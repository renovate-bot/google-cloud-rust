// Copyright 2025 Google LLC
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
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Container Analysis API.
///
/// # Service Description
///
/// Retrieves analysis results of Cloud components such as Docker container
/// images. The Container Analysis API is an implementation of the
/// [Grafeas](https://grafeas.io) API.
///
/// Analysis results are stored as a series of occurrences. An `Occurrence`
/// contains information about a specific analysis instance on a resource. An
/// occurrence refers to a `Note`. A note contains details describing the
/// analysis and is generally stored in a separate project, called a `Provider`.
/// Multiple occurrences can refer to the same note.
///
/// For example, an SSL vulnerability could affect multiple images. In this case,
/// there would be one note for the vulnerability and an occurrence for each
/// image with the vulnerability referring to that note.
///
/// # Configuration
///
/// `ContainerAnalysis` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ContainerAnalysis` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ContainerAnalysis` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ContainerAnalysis {
    inner: Arc<dyn super::stub::dynamic::ContainerAnalysis>,
}

impl ContainerAnalysis {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::ContainerAnalysis + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ContainerAnalysis>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ContainerAnalysis> {
        super::transport::ContainerAnalysis::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ContainerAnalysis> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ContainerAnalysis::new)
    }

    /// Sets the access control policy on the specified note or occurrence.
    /// Requires `containeranalysis.notes.setIamPolicy` or
    /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
    /// a note or an occurrence, respectively.
    ///
    /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
    /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
    /// occurrences.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::container_analysis::SetIamPolicy {
        super::builder::container_analysis::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a note or an occurrence resource.
    /// Requires `containeranalysis.notes.setIamPolicy` or
    /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
    /// a note or occurrence, respectively.
    ///
    /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
    /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
    /// occurrences.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::container_analysis::GetIamPolicy {
        super::builder::container_analysis::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns the permissions that a caller has on the specified note or
    /// occurrence. Requires list permission on the project (for example,
    /// `containeranalysis.notes.list`).
    ///
    /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
    /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
    /// occurrences.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::container_analysis::TestIamPermissions {
        super::builder::container_analysis::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets a summary of the number and severity of occurrences.
    pub fn get_vulnerability_occurrences_summary(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::container_analysis::GetVulnerabilityOccurrencesSummary {
        super::builder::container_analysis::GetVulnerabilityOccurrencesSummary::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }
}
