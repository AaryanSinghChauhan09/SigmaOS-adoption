//! SigmaOS-adoption — comprehensive adoption tracking component.
//!
//! Provides technology adoption tracking for monitoring feature usage,
//! user adoption metrics, and technology lifecycle management.

use std::collections::HashMap;

/// Technology category
#[derive(Debug, Clone, PartialEq)]
pub enum TechnologyCategory {
    /// Programming language
    ProgrammingLanguage,
    /// Framework
    Framework,
    /// Library
    Library,
    /// Tool
    Tool,
    /// Platform
    Platform,
    /// Service
    Service,
    /// Protocol
    Protocol,
    /// Standard
    Standard,
}

/// Adoption stage
#[derive(Debug, Clone, PartialEq)]
pub enum AdoptionStage {
    /// Early adopters
    EarlyAdopters,
    /// Early majority
    EarlyMajority,
    /// Late majority
    LateMajority,
    /// Laggards
    Laggards,
    /// Not adopted
    NotAdopted,
}

/// Technology adoption record
#[derive(Debug, Clone)]
pub struct TechnologyAdoption {
    /// Technology name
    pub name: String,
    /// Technology category
    pub category: TechnologyCategory,
    /// Adoption stage
    pub stage: AdoptionStage,
    /// Adoption percentage
    pub adoption_percentage: f64,
    /// User count
    pub user_count: u64,
    /// Organization count
    pub organization_count: u64,
    /// First adopted date
    pub first_adopted: u64,
    /// Last updated date
    pub last_updated: u64,
    /// Description
    pub description: String,
    /// Website
    pub website: String,
    /// License
    pub license: String,
    /// Version
    pub version: String,
}

/// User adoption profile
#[derive(Debug, Clone)]
pub struct UserAdoptionProfile {
    /// User ID
    pub user_id: String,
    /// User name
    pub user_name: String,
    /// Technologies adopted
    pub technologies: Vec<String>,
    /// Adoption stage
    pub stage: AdoptionStage,
    /// Adoption score
    pub adoption_score: f64,
    /// Last active date
    pub last_active: u64,
    /// Preferred technologies
    pub preferred_technologies: Vec<String>,
}

/// Organization adoption profile
#[derive(Debug, Clone)]
pub struct OrganizationAdoptionProfile {
    /// Organization ID
    pub org_id: String,
    /// Organization name
    pub org_name: String,
    /// Technologies adopted
    pub technologies: Vec<String>,
    /// Adoption stage
    pub stage: AdoptionStage,
    /// Adoption score
    pub adoption_score: f64,
    /// Industry
    pub industry: String,
    /// Size
    pub size: String,
    /// Location
    pub location: String,
}

/// Adoption metric
#[derive(Debug, Clone)]
pub struct AdoptionMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric type
    pub metric_type: MetricType,
    /// Timestamp
    pub timestamp: u64,
    /// Description
    pub description: String,
}

/// Metric type
#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    /// Percentage
    Percentage,
    /// Count
    Count,
    /// Rate
    Rate,
    /// Score
    Score,
}

/// Adoption tracker
#[derive(Debug, Clone)]
pub struct AdoptionTracker {
    /// Technology adoptions
    pub technologies: HashMap<String, TechnologyAdoption>,
    /// User profiles
    pub user_profiles: HashMap<String, UserAdoptionProfile>,
    /// Organization profiles
    pub org_profiles: HashMap<String, OrganizationAdoptionProfile>,
    /// Adoption metrics
    pub metrics: Vec<AdoptionMetric>,
    /// Statistics
    pub stats: AdoptionStatistics,
}

/// Adoption statistics
#[derive(Debug, Clone, Default)]
pub struct AdoptionStatistics {
    /// Total technologies
    pub total_technologies: u64,
    /// Total users
    pub total_users: u64,
    /// Total organizations
    pub total_organizations: u64,
    /// Average adoption percentage
    pub avg_adoption_percentage: f64,
    /// Total adoption score
    pub total_adoption_score: f64,
    /// Last updated
    pub last_updated: u64,
}

impl Default for AdoptionTracker {
    fn default() -> Self {
        Self {
            technologies: HashMap::new(),
            user_profiles: HashMap::new(),
            org_profiles: HashMap::new(),
            metrics: Vec::new(),
            stats: AdoptionStatistics::default(),
        }
    }
}

impl AdoptionTracker {
    /// Create new adoption tracker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with sample data
    pub fn initialize(&mut self) {
        // Add sample technologies
        let technologies = vec![
            ("Rust", TechnologyCategory::ProgrammingLanguage, AdoptionStage::EarlyMajority, 45.0),
            ("Python", TechnologyCategory::ProgrammingLanguage, AdoptionStage::LateMajority, 85.0),
            ("JavaScript", TechnologyCategory::ProgrammingLanguage, AdoptionStage::LateMajority, 90.0),
            ("React", TechnologyCategory::Framework, AdoptionStage::EarlyMajority, 60.0),
            ("Vue.js", TechnologyCategory::Framework, AdoptionStage::EarlyMajority, 40.0),
            ("Docker", TechnologyCategory::Tool, AdoptionStage::LateMajority, 75.0),
            ("Kubernetes", TechnologyCategory::Platform, AdoptionStage::EarlyMajority, 55.0),
            ("Git", TechnologyCategory::Tool, AdoptionStage::LateMajority, 95.0),
        ];
        
        for (name, category, stage, percentage) in technologies {
            let adoption = TechnologyAdoption {
                name: name.to_string(),
                category,
                stage,
                adoption_percentage: percentage,
                user_count: (percentage * 10.0) as u64,
                organization_count: (percentage * 5.0) as u64,
                first_adopted: 1609459200, // 2021-01-01
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                description: format!("{} technology", name),
                website: format!("https://{}.com", name.to_lowercase().replace(' ', "")),
                license: "Open Source".into(),
                version: "1.0".into(),
            };
            self.technologies.insert(name.to_string(), adoption);
        }
        
        self.update_statistics();
    }
    
    /// Add technology adoption
    pub fn add_technology(&mut self, adoption: TechnologyAdoption) -> String {
        let name = adoption.name.clone();
        self.technologies.insert(name.clone(), adoption);
        self.update_statistics();
        name
    }
    
    /// Remove technology
    pub fn remove_technology(&mut self, name: &str) -> bool {
        if self.technologies.remove(name).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get technology
    pub fn get_technology(&self, name: &str) -> Option<&TechnologyAdoption> {
        self.technologies.get(name)
    }
    
    /// Get all technologies
    pub fn get_all_technologies(&self) -> Vec<&TechnologyAdoption> {
        self.technologies.values().collect()
    }
    
    /// Get technologies by category
    pub fn get_technologies_by_category(&self, category: &TechnologyCategory) -> Vec<&TechnologyAdoption> {
        self.technologies.values()
            .filter(|t| t.category == *category)
            .collect()
    }
    
    /// Get technologies by stage
    pub fn get_technologies_by_stage(&self, stage: &AdoptionStage) -> Vec<&TechnologyAdoption> {
        self.technologies.values()
            .filter(|t| t.stage == *stage)
            .collect()
    }
    
    /// Update technology adoption
    pub fn update_technology_adoption(&mut self, name: &str, adoption_percentage: f64, user_count: u64, org_count: u64) -> bool {
        if let Some(tech) = self.technologies.get_mut(name) {
            tech.adoption_percentage = adoption_percentage;
            tech.user_count = user_count;
            tech.organization_count = org_count;
            tech.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Update stage based on percentage
            tech.stage = match adoption_percentage {
                p if p < 16.0 => AdoptionStage::EarlyAdopters,
                p if p < 50.0 => AdoptionStage::EarlyMajority,
                p if p < 84.0 => AdoptionStage::LateMajority,
                p if p < 100.0 => AdoptionStage::Laggards,
                _ => AdoptionStage::NotAdopted,
            };
            
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add user profile
    pub fn add_user_profile(&mut self, profile: UserAdoptionProfile) -> String {
        let user_id = profile.user_id.clone();
        self.user_profiles.insert(user_id.clone(), profile);
        self.update_statistics();
        user_id
    }
    
    /// Get user profile
    pub fn get_user_profile(&self, user_id: &str) -> Option<&UserAdoptionProfile> {
        self.user_profiles.get(user_id)
    }
    
    /// Get all user profiles
    pub fn get_all_user_profiles(&self) -> Vec<&UserAdoptionProfile> {
        self.user_profiles.values().collect()
    }
    
    /// Add organization profile
    pub fn add_org_profile(&mut self, profile: OrganizationAdoptionProfile) -> String {
        let org_id = profile.org_id.clone();
        self.org_profiles.insert(org_id.clone(), profile);
        self.update_statistics();
        org_id
    }
    
    /// Get organization profile
    pub fn get_org_profile(&self, org_id: &str) -> Option<&OrganizationAdoptionProfile> {
        self.org_profiles.get(org_id)
    }
    
    /// Get all organization profiles
    pub fn get_all_org_profiles(&self) -> Vec<&OrganizationAdoptionProfile> {
        self.org_profiles.values().collect()
    }
    
    /// Add adoption metric
    pub fn add_metric(&mut self, metric: AdoptionMetric) {
        self.metrics.push(metric);
    }
    
    /// Get metrics by type
    pub fn get_metrics_by_type(&self, metric_type: &MetricType) -> Vec<&AdoptionMetric> {
        self.metrics.iter()
            .filter(|m| m.metric_type == *metric_type)
            .collect()
    }
    
    /// Get latest metrics
    pub fn get_latest_metrics(&self, limit: usize) -> Vec<&AdoptionMetric> {
        let mut metrics: Vec<&AdoptionMetric> = self.metrics.iter().collect();
        metrics.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        metrics.truncate(limit);
        metrics
    }
    
    /// Search technologies
    pub fn search_technologies(&self, query: &str) -> Vec<&TechnologyAdoption> {
        let query_lower = query.to_lowercase();
        self.technologies.values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower) ||
                t.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Get adoption trends
    pub fn get_adoption_trends(&self, technology: &str) -> Vec<AdoptionMetric> {
        self.metrics.iter()
            .filter(|m| m.name == technology)
            .cloned()
            .collect()
    }
    
    /// Calculate adoption score
    pub fn calculate_adoption_score(&self) -> f64 {
        if self.technologies.is_empty() {
            return 0.0;
        }
        
        let total_percentage: f64 = self.technologies.values()
            .map(|t| t.adoption_percentage)
            .sum();
        
        total_percentage / self.technologies.len() as f64
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.stats.total_technologies = self.technologies.len() as u64;
        self.stats.total_users = self.user_profiles.len() as u64;
        self.stats.total_organizations = self.org_profiles.len() as u64;
        
        if !self.technologies.is_empty() {
            self.stats.avg_adoption_percentage = self.technologies.values()
                .map(|t| t.adoption_percentage)
                .sum::<f64>() / self.technologies.len() as f64;
        }
        
        self.stats.total_adoption_score = self.calculate_adoption_score();
        self.stats.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get adoption statistics
    pub fn get_statistics(&self) -> &AdoptionStatistics {
        &self.stats
    }
    
    /// Export adoption data
    pub fn export_data(&self) -> AdoptionExport {
        AdoptionExport {
            technologies: self.technologies.values().cloned().collect(),
            user_profiles: self.user_profiles.values().cloned().collect(),
            org_profiles: self.org_profiles.values().cloned().collect(),
            metrics: self.metrics.clone(),
        }
    }
    
    /// Import adoption data
    pub fn import_data(&mut self, export: AdoptionExport) {
        for tech in export.technologies {
            self.technologies.insert(tech.name.clone(), tech);
        }
        
        for profile in export.user_profiles {
            self.user_profiles.insert(profile.user_id.clone(), profile);
        }
        
        for profile in export.org_profiles {
            self.org_profiles.insert(profile.org_id.clone(), profile);
        }
        
        self.metrics = export.metrics;
        self.update_statistics();
    }
}

/// Adoption export data
#[derive(Debug, Clone)]
pub struct AdoptionExport {
    /// Technologies
    pub technologies: Vec<TechnologyAdoption>,
    /// User profiles
    pub user_profiles: Vec<UserAdoptionProfile>,
    /// Organization profiles
    pub org_profiles: Vec<OrganizationAdoptionProfile>,
    /// Metrics
    pub metrics: Vec<AdoptionMetric>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_technology_category() {
        let categories = vec![
            TechnologyCategory::ProgrammingLanguage,
            TechnologyCategory::Framework,
            TechnologyCategory::Library,
            TechnologyCategory::Tool,
            TechnologyCategory::Platform,
            TechnologyCategory::Service,
            TechnologyCategory::Protocol,
            TechnologyCategory::Standard,
        ];
        assert_eq!(categories.len(), 8);
    }
    
    #[test]
    fn test_adoption_stage() {
        let stages = vec![
            AdoptionStage::EarlyAdopters,
            AdoptionStage::EarlyMajority,
            AdoptionStage::LateMajority,
            AdoptionStage::Laggards,
            AdoptionStage::NotAdopted,
        ];
        assert_eq!(stages.len(), 5);
    }
    
    #[test]
    fn test_technology_adoption_creation() {
        let adoption = TechnologyAdoption {
            name: "Rust".into(),
            category: TechnologyCategory::ProgrammingLanguage,
            stage: AdoptionStage::EarlyMajority,
            adoption_percentage: 45.0,
            user_count: 450,
            organization_count: 225,
            first_adopted: 1609459200,
            last_updated: 1234567890,
            description: "Rust programming language".into(),
            website: "https://rust-lang.org".into(),
            license: "MIT/Apache-2.0".into(),
            version: "1.70".into(),
        };
        
        assert_eq!(adoption.name, "Rust");
        assert_eq!(adoption.adoption_percentage, 45.0);
    }
    
    #[test]
    fn test_tracker_creation() {
        let tracker = AdoptionTracker::new();
        assert_eq!(tracker.get_statistics().total_technologies, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        assert!(tracker.get_statistics().total_technologies > 0);
        assert!(tracker.get_technology("Rust").is_some());
    }
    
    #[test]
    fn test_add_technology() {
        let mut tracker = AdoptionTracker::new();
        
        let adoption = TechnologyAdoption {
            name: "Test Tech".into(),
            category: TechnologyCategory::Tool,
            stage: AdoptionStage::EarlyAdopters,
            adoption_percentage: 15.0,
            user_count: 150,
            organization_count: 75,
            first_adopted: 1609459200,
            last_updated: 1234567890,
            description: "Test technology".into(),
            website: "https://test.com".into(),
            license: "MIT".into(),
            version: "1.0".into(),
        };
        
        tracker.add_technology(adoption);
        assert!(tracker.get_technology("Test Tech").is_some());
        assert_eq!(tracker.get_statistics().total_technologies, 1);
    }
    
    #[test]
    fn test_get_technologies_by_category() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        let languages = tracker.get_technologies_by_category(&TechnologyCategory::ProgrammingLanguage);
        assert!(!languages.is_empty());
    }
    
    #[test]
    fn test_get_technologies_by_stage() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        let early_majority = tracker.get_technologies_by_stage(&AdoptionStage::EarlyMajority);
        assert!(!early_majority.is_empty());
    }
    
    #[test]
    fn test_update_technology_adoption() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        assert!(tracker.update_technology_adoption("Rust", 40.0, 400, 200));
        
        let rust = tracker.get_technology("Rust").unwrap();
        assert_eq!(rust.adoption_percentage, 40.0);
        assert_eq!(rust.user_count, 400);
        assert_eq!(rust.stage, AdoptionStage::EarlyMajority);
    }
    
    #[test]
    fn test_user_profile() {
        let mut tracker = AdoptionTracker::new();
        
        let profile = UserAdoptionProfile {
            user_id: "user1".into(),
            user_name: "John Doe".into(),
            technologies: vec!["Rust".into(), "Python".into()],
            stage: AdoptionStage::EarlyMajority,
            adoption_score: 75.0,
            last_active: 1234567890,
            preferred_technologies: vec!["Rust".into()],
        };
        
        tracker.add_user_profile(profile);
        assert!(tracker.get_user_profile("user1").is_some());
        assert_eq!(tracker.get_statistics().total_users, 1);
    }
    
    #[test]
    fn test_org_profile() {
        let mut tracker = AdoptionTracker::new();
        
        let profile = OrganizationAdoptionProfile {
            org_id: "org1".into(),
            org_name: "Acme Corp".into(),
            technologies: vec!["Python".into(), "JavaScript".into()],
            stage: AdoptionStage::LateMajority,
            adoption_score: 85.0,
            industry: "Technology".into(),
            size: "Large".into(),
            location: "San Francisco".into(),
        };
        
        tracker.add_org_profile(profile);
        assert!(tracker.get_org_profile("org1").is_some());
        assert_eq!(tracker.get_statistics().total_organizations, 1);
    }
    
    #[test]
    fn test_search_technologies() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        let results = tracker.search_technologies("rust");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "Rust");
    }
    
    #[test]
    fn test_calculate_adoption_score() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        let score = tracker.calculate_adoption_score();
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }
    
    #[test]
    fn test_export_import() {
        let mut tracker = AdoptionTracker::new();
        tracker.initialize();
        
        let export = tracker.export_data();
        
        let mut tracker2 = AdoptionTracker::new();
        tracker2.import_data(export);
        
        assert_eq!(tracker2.get_statistics().total_technologies, tracker.get_statistics().total_technologies);
    }
}
