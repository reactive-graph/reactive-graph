pub enum RelationTypeCardinality {
    /// Exactly one relation instance of a relation type can exist between two entity instances.
    Unique,

    /// Multiple relation instances of a relation type can exist between two entity instances. The
    /// instance_id must be specified.
    Multiple,

    /// Multiple relation instances of a relation type can exist between two entity instances. The
    /// instance_id is generated randomly.
    Random,
}
