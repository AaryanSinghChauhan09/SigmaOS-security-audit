// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-security-audit: security audit component model.
//!
//! Real implementation: audits, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audit {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityAuditError {
    EmptyAuditName,
    DuplicateAudit(String),
    EmptyEventSubject,
}

impl fmt::Display for SecurityAuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAuditName => write!(f, "audit name must not be empty"),
            Self::DuplicateAudit(id) => write!(f, "duplicate audit {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SecurityAuditError {}

pub struct SecurityAudit {
    pub name: String,
    audits: BTreeMap<String, Audit>,
    events: BTreeMap<String, Event>,
}

impl SecurityAudit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            audits: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_audit(&mut self, item: Audit) -> Result<(), SecurityAuditError> {
        if item.name.trim().is_empty() {
            return Err(SecurityAuditError::EmptyAuditName);
        }
        let id = item.id.clone();
        if self.audits.contains_key(&id) {
            return Err(SecurityAuditError::DuplicateAudit(id.clone()));
        }
        self.audits.insert(id, item);
        Ok(())
    }

    pub fn audit_count(&self) -> usize {
        self.audits.len()
    }

    pub fn get_audit(&self, id: &str) -> Option<&Audit> {
        self.audits.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SecurityAuditError> {
        if event.subject.trim().is_empty() {
            return Err(SecurityAuditError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_audit(id: &str, name: &str) -> Audit {
        Audit {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SecurityAudit::new("SecurityAudit");
        assert_eq!(m.name, "SecurityAudit");
        assert_eq!(m.audit_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_audit() {
        let mut m = SecurityAudit::new("SecurityAudit");
        m.add_audit(sample_audit("a1", "Alpha")).unwrap();
        assert_eq!(m.audit_count(), 1);
        assert!(m.get_audit("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SecurityAudit::new("SecurityAudit");
        assert!(m.add_audit(sample_audit("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SecurityAudit::new("SecurityAudit");
        m.add_audit(sample_audit("x", "One")).unwrap();
        assert!(m.add_audit(sample_audit("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SecurityAudit::new("SecurityAudit");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
