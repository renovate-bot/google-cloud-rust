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

/// Implements a client for the Web Risk API.
///
/// # Service Description
///
/// Web Risk API defines an interface to detect malicious URLs on your
/// website and in client applications.
///
/// # Configuration
///
/// `WebRiskService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `WebRiskService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `WebRiskService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct WebRiskService {
    inner: Arc<dyn super::stub::dynamic::WebRiskService>,
}

impl WebRiskService {
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
        T: super::stub::WebRiskService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::WebRiskService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::WebRiskService> {
        super::transport::WebRiskService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::WebRiskService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::WebRiskService::new)
    }

    /// Gets the most recent threat list diffs. These diffs should be applied to
    /// a local database of hashes to keep it up-to-date. If the local database is
    /// empty or excessively out-of-date, a complete snapshot of the database will
    /// be returned. This Method only updates a single ThreatList at a time. To
    /// update multiple ThreatList databases, this method needs to be called once
    /// for each list.
    pub fn compute_threat_list_diff(
        &self,
    ) -> super::builder::web_risk_service::ComputeThreatListDiff {
        super::builder::web_risk_service::ComputeThreatListDiff::new(self.inner.clone())
    }

    /// This method is used to check whether a URI is on a given threatList.
    /// Multiple threatLists may be searched in a single query.
    /// The response will list all requested threatLists the URI was found to
    /// match. If the URI is not found on any of the requested ThreatList an
    /// empty response will be returned.
    pub fn search_uris(&self) -> super::builder::web_risk_service::SearchUris {
        super::builder::web_risk_service::SearchUris::new(self.inner.clone())
    }

    /// Gets the full hashes that match the requested hash prefix.
    /// This is used after a hash prefix is looked up in a threatList
    /// and there is a match. The client side threatList only holds partial hashes
    /// so the client must query this method to determine if there is a full
    /// hash match of a threat.
    pub fn search_hashes(&self) -> super::builder::web_risk_service::SearchHashes {
        super::builder::web_risk_service::SearchHashes::new(self.inner.clone())
    }

    /// Creates a Submission of a URI suspected of containing phishing content to
    /// be reviewed. If the result verifies the existence of malicious phishing
    /// content, the site will be added to the [Google's Social Engineering
    /// lists](https://support.google.com/webmasters/answer/6350487/) in order to
    /// protect users that could get exposed to this threat in the future. Only
    /// allowlisted projects can use this method during Early Access. Please reach
    /// out to Sales or your customer engineer to obtain access.
    pub fn create_submission(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::CreateSubmission {
        super::builder::web_risk_service::CreateSubmission::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Submits a URI suspected of containing malicious content to be reviewed.
    /// Returns a google.longrunning.Operation which, once the review is complete,
    /// is updated with its result. You can use the [Pub/Sub API]
    /// (<https://cloud.google.com/pubsub>) to receive notifications for the returned
    /// Operation. If the result verifies the existence of malicious content, the
    /// site will be added to the [Google's Social Engineering lists]
    /// (<https://support.google.com/webmasters/answer/6350487/>) in order to
    /// protect users that could get exposed to this threat in the future. Only
    /// allowlisted projects can use this method during Early Access. Please reach
    /// out to Sales or your customer engineer to obtain access.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn submit_uri(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::SubmitUri {
        super::builder::web_risk_service::SubmitUri::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::ListOperations {
        super::builder::web_risk_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::GetOperation {
        super::builder::web_risk_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::DeleteOperation {
        super::builder::web_risk_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::web_risk_service::CancelOperation {
        super::builder::web_risk_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
