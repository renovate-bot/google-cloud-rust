// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package openapi reads OpenAPI v3 specifications and converts them into
// the `genclient` model.
package openapi

import (
	"fmt"
	"strings"

	"github.com/googleapis/google-cloud-rust/generator/internal/genclient"
	"github.com/googleapis/google-cloud-rust/generator/internal/genclient/language"
	"github.com/pb33f/libopenapi"
	base "github.com/pb33f/libopenapi/datamodel/high/base"
	v3 "github.com/pb33f/libopenapi/datamodel/high/v3"
)

type Translator struct {
	model    *libopenapi.DocumentModel[v3.Document]
	language string

	// State by FQN
	state *genclient.APIState

	// Only used for local testing
	outDir      string
	templateDir string
}

type Options struct {
	Language string
	// Only used for local testing
	OutDir      string
	TemplateDir string
}

func NewTranslator(contents []byte, opts *Options) (*Translator, error) {
	document, err := libopenapi.NewDocument(contents)
	if err != nil {
		return nil, err
	}
	docModel, errors := document.BuildV3Model()
	if len(errors) > 0 {
		for i := range errors {
			fmt.Printf("error: %e\n", errors[i])
		}
		return nil, fmt.Errorf("cannot convert document to OpenAPI V3 model: %e", errors[0])
	}

	return &Translator{
		model:       docModel,
		outDir:      opts.OutDir,
		language:    opts.Language,
		templateDir: opts.TemplateDir,
		state: &genclient.APIState{
			ServiceByID: make(map[string]*genclient.Service),
			MessageByID: make(map[string]*genclient.Message),
			EnumByID:    make(map[string]*genclient.Enum),
		},
	}, nil
}

func (t *Translator) makeAPI() (*genclient.API, error) {
	api := &genclient.API{
		Name:     t.model.Model.Info.Title,
		Messages: make([]*genclient.Message, 0),
	}
	for name, msg := range t.model.Model.Components.Schemas.FromOldest() {
		schema, err := msg.BuildSchema()
		if err != nil {
			return nil, err
		}
		fields, err := t.makeMessageFields(name, schema)
		if err != nil {
			return nil, err
		}
		message := genclient.Message{
			Name:          name,
			Documentation: msg.Schema().Description,
			Fields:        fields,
		}

		api.Messages = append(api.Messages, &message)
	}
	return api, nil
}

// Translates OpenAPI specification into a [genclient.GenerateRequest].
func (t *Translator) Translate() (*genclient.GenerateRequest, error) {
	api, err := t.makeAPI()
	if err != nil {
		return nil, err
	}

	codec, err := language.NewCodec(t.language)
	if err != nil {
		return nil, err
	}
	api.State = t.state
	return &genclient.GenerateRequest{
		API:         api,
		Codec:       codec,
		OutDir:      t.outDir,
		TemplateDir: t.templateDir,
	}, nil
}

func (t *Translator) makeMessageFields(messageName string, message *base.Schema) ([]*genclient.Field, error) {
	var fields []*genclient.Field
	for name, f := range message.Properties.FromOldest() {
		schema, err := f.BuildSchema()
		if err != nil {
			return nil, err
		}
		optional := true
		for _, r := range message.Required {
			if name == r {
				optional = false
				break
			}
		}
		field, err := t.makeField(messageName, name, optional, schema)
		if err != nil {
			return nil, err
		}
		fields = append(fields, field)
	}
	return fields, nil
}

func (t *Translator) makeField(messageName, name string, optional bool, field *base.Schema) (*genclient.Field, error) {
	if len(field.AllOf) != 0 {
		// Simple object fields name an AllOf attribute, but no `Type` attribute.
		return t.makeObjectField(messageName, name, field)
	}
	if len(field.Type) == 0 {
		return nil, fmt.Errorf("missing field type for field %s.%s", messageName, name)
	}
	switch field.Type[0] {
	case "boolean":
		return t.makeScalarField(messageName, name, field, optional, field)
	case "integer":
		return t.makeScalarField(messageName, name, field, optional, field)
	case "number":
		return t.makeScalarField(messageName, name, field, optional, field)
	case "string":
		return t.makeScalarField(messageName, name, field, optional, field)
	case "object":
		return t.makeObjectField(messageName, name, field)
	case "array":
		return t.makeArrayField(messageName, name, field)
	default:
		return nil, fmt.Errorf("unknown type for field %q", name)
	}
}

func (t *Translator) makeScalarField(messageName, name string, schema *base.Schema, optional bool, field *base.Schema) (*genclient.Field, error) {
	typez, typezID, err := scalarType(messageName, name, schema)
	if err != nil {
		return nil, err
	}
	return &genclient.Field{
		Name:          name,
		Documentation: field.Description,
		Typez:         typez,
		TypezID:       typezID,
		Optional:      optional || (typez == genclient.MESSAGE_TYPE),
	}, nil
}

func (t *Translator) makeObjectField(messageName, name string, field *base.Schema) (*genclient.Field, error) {
	if len(field.AllOf) != 0 {
		return t.makeObjectFieldAllOf(messageName, name, field)
	}
	// TODO(#62) - this is an Any or a map<string, T>, needs a TypezID
	return &genclient.Field{
		Name:          name,
		Documentation: field.Description,
		Typez:         genclient.MESSAGE_TYPE,
		Optional:      true,
	}, nil
}

func (t *Translator) makeArrayField(messageName, name string, field *base.Schema) (*genclient.Field, error) {
	if !field.Items.IsA() {
		return nil, fmt.Errorf("cannot handle arrays without an `Items` field for %s.%s", messageName, name)
	}
	schema, err := field.Items.A.BuildSchema()
	if err != nil {
		return nil, fmt.Errorf("cannot build items schema for %s.%s error=%q", messageName, name, err)
	}
	if len(schema.Type) != 1 {
		return nil, fmt.Errorf("the items for field  %s.%s should have a single type", messageName, name)
	}
	var result *genclient.Field
	switch schema.Type[0] {
	case "boolean":
		result, err = t.makeScalarField(messageName, name, schema, false, field)
	case "integer":
		result, err = t.makeScalarField(messageName, name, schema, false, field)
	case "number":
		result, err = t.makeScalarField(messageName, name, schema, false, field)
	case "string":
		result, err = t.makeScalarField(messageName, name, schema, false, field)
	case "object":
		result, err = t.makeObjectField(messageName, name, field)
	default:
		return nil, fmt.Errorf("unknown array field type for %s.%s %q", messageName, name, schema.Type[0])
	}
	if err != nil {
		return nil, err
	}
	result.Repeated = true
	result.Optional = false
	return result, nil
}

func (t *Translator) makeObjectFieldAllOf(messageName, name string, field *base.Schema) (*genclient.Field, error) {
	for _, proxy := range field.AllOf {
		typezID := strings.TrimPrefix(proxy.GetReference(), "#/components/schemas/")
		return &genclient.Field{
			Name:          name,
			Documentation: field.Description,
			Typez:         genclient.MESSAGE_TYPE,
			TypezID:       typezID,
			Optional:      true,
		}, nil
	}
	return nil, fmt.Errorf("cannot build any AllOf schema for field %s.%s", messageName, name)
}

func scalarType(messageName, name string, schema *base.Schema) (genclient.Typez, string, error) {
	for _, type_name := range schema.Type {
		switch type_name {
		case "boolean":
			return genclient.BOOL_TYPE, "bool", nil
		case "integer":
			return scalarTypeForIntegerFormats(messageName, name, schema)
		case "number":
			return scalarTypeForNumberFormats(messageName, name, schema)
		case "string":
			return scalarTypeForStringFormats(messageName, name, schema)
		}
	}
	return 0, "", fmt.Errorf("expected a scalar type for field %s.%s", messageName, name)
}

func scalarTypeForIntegerFormats(messageName, name string, schema *base.Schema) (genclient.Typez, string, error) {
	switch schema.Format {
	case "int32":
		if schema.Minimum != nil && *schema.Minimum == 0 {
			return genclient.UINT32_TYPE, "uint32", nil
		}
		return genclient.INT32_TYPE, "int32", nil
	case "int64":
		if schema.Minimum != nil && *schema.Minimum == 0 {
			return genclient.UINT64_TYPE, "uint64", nil
		}
		return genclient.INT64_TYPE, "int64", nil
	}
	return 0, "", fmt.Errorf("unknown integer format (%s) for field %s.%s", schema.Format, messageName, name)
}

func scalarTypeForNumberFormats(messageName, name string, schema *base.Schema) (genclient.Typez, string, error) {
	switch schema.Format {
	case "float":
		return genclient.FLOAT_TYPE, "float", nil
	case "double":
		return genclient.DOUBLE_TYPE, "double", nil
	}
	return 0, "", fmt.Errorf("unknown number format (%s) for field %s.%s", schema.Format, messageName, name)
}

func scalarTypeForStringFormats(messageName, name string, schema *base.Schema) (genclient.Typez, string, error) {
	switch schema.Format {
	case "":
		return genclient.STRING_TYPE, "string", nil
	case "byte":
		return genclient.BYTES_TYPE, "bytes", nil
	case "int32":
		if schema.Minimum != nil && *schema.Minimum == 0 {
			return genclient.UINT32_TYPE, "uint32", nil
		}
		return genclient.INT32_TYPE, "int32", nil
	case "int64":
		if schema.Minimum != nil && *schema.Minimum == 0 {
			return genclient.UINT64_TYPE, "uint64", nil
		}
		return genclient.INT64_TYPE, "int64", nil
	case "google-duration":
		return genclient.MESSAGE_TYPE, ".google.protobuf.Duration", nil
	case "date-time":
		return genclient.MESSAGE_TYPE, ".google.protobuf.Timestamp", nil
	case "google-fieldmask":
		return genclient.MESSAGE_TYPE, ".google.protobuf.FieldMask", nil
	}
	return 0, "", fmt.Errorf("unknown string format (%s) for field %s.%s", schema.Format, messageName, name)
}
