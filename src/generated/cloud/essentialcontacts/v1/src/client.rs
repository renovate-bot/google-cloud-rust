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

/// Implements a client for the Essential Contacts API.
///
/// # Service Description
///
/// Manages contacts for important Google Cloud notifications.
///
/// # Configuration
///
/// `EssentialContactsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `EssentialContactsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `EssentialContactsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct EssentialContactsService {
    inner: Arc<dyn super::stub::dynamic::EssentialContactsService>,
}

impl EssentialContactsService {
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
        T: super::stub::EssentialContactsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::EssentialContactsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::EssentialContactsService> {
        super::transport::EssentialContactsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::EssentialContactsService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::EssentialContactsService::new)
    }

    /// Adds a new contact for a resource.
    pub fn create_contact(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::CreateContact {
        super::builder::essential_contacts_service::CreateContact::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a contact.
    /// Note: A contact's email address cannot be changed.
    pub fn update_contact(
        &self,
        contact: impl Into<crate::model::Contact>,
    ) -> super::builder::essential_contacts_service::UpdateContact {
        super::builder::essential_contacts_service::UpdateContact::new(self.inner.clone())
            .set_contact(contact.into())
    }

    /// Lists the contacts that have been set on a resource.
    pub fn list_contacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::ListContacts {
        super::builder::essential_contacts_service::ListContacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a single contact.
    pub fn get_contact(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::GetContact {
        super::builder::essential_contacts_service::GetContact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a contact.
    pub fn delete_contact(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::DeleteContact {
        super::builder::essential_contacts_service::DeleteContact::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all contacts for the resource that are subscribed to the
    /// specified notification categories, including contacts inherited from
    /// any parent resources.
    pub fn compute_contacts(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::ComputeContacts {
        super::builder::essential_contacts_service::ComputeContacts::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Allows a contact admin to send a test message to contact to verify that it
    /// has been configured correctly.
    pub fn send_test_message(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::essential_contacts_service::SendTestMessage {
        super::builder::essential_contacts_service::SendTestMessage::new(self.inner.clone())
            .set_resource(resource.into())
    }
}
