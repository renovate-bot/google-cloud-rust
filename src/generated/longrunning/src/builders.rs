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
//
// Code generated by sidekick. DO NOT EDIT.

use crate::Result;
use std::sync::Arc;

/// Common implementation for [crate::client::Operations] request builders.
#[derive(Clone, Debug)]
pub struct OperationsRequestBuilder<R: std::default::Default> {
    stub: Arc<dyn crate::traits::dyntraits::Operations>,
    request: R,
    options: gax::options::RequestOptions,
}

impl<R> OperationsRequestBuilder<R>
where
    R: std::default::Default,
{
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self {
            stub,
            request: R::default(),
            options: gax::options::RequestOptions::default(),
        }
    }
}

/// The request builder for a Operations::list_operations call.
#[derive(Clone, Debug)]
pub struct ListOperations(OperationsRequestBuilder<crate::model::ListOperationsRequest>);

impl ListOperations {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::ListOperationsRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<crate::model::ListOperationsResponse> {
        self.0
            .stub
            .list_operations(self.0.request, self.0.options)
            .await
    }

    /// Streams the responses back.
    #[cfg(feature = "unstable-stream")]
    pub async fn stream(
        self,
    ) -> gax::paginator::Paginator<crate::model::ListOperationsResponse, gax::error::Error> {
        let token = gax::paginator::extract_token(&self.0.request.page_token);
        let execute = move |token: String| {
            let builder = self.clone();
            builder.0.request.clone().set_page_token(token);
            builder.send()
        };
        gax::paginator::Paginator::new(token, execute)
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.0.request.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.page_token = v.into();
        self
    }
}

impl gax::options::RequestBuilder for ListOperations {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Operations::get_operation call.
#[derive(Clone, Debug)]
pub struct GetOperation(OperationsRequestBuilder<crate::model::GetOperationRequest>);

impl GetOperation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::GetOperationRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<crate::model::Operation> {
        self.0
            .stub
            .get_operation(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for GetOperation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Operations::delete_operation call.
#[derive(Clone, Debug)]
pub struct DeleteOperation(OperationsRequestBuilder<crate::model::DeleteOperationRequest>);

impl DeleteOperation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::DeleteOperationRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<wkt::Empty> {
        self.0
            .stub
            .delete_operation(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for DeleteOperation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Operations::cancel_operation call.
#[derive(Clone, Debug)]
pub struct CancelOperation(OperationsRequestBuilder<crate::model::CancelOperationRequest>);

impl CancelOperation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::CancelOperationRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<wkt::Empty> {
        self.0
            .stub
            .cancel_operation(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for CancelOperation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}