* FIXME do I need Clone traits everywhere?
* FIXME make all errors specific
* FIXME extend error descriptions
* FIXME rename "internal" to something that makes more sense
* FIXME get rid of telemetry
* FIXME ensure that oneof "enums" get only one field set
* FIXME prost.Timestamp to datetime instead of u64
* FIXME in "if let ..." cases, make compiler verify that all fields get tested somehow
* FIXME parse SDK's exceptions and convert them to specific Python exceptions
* FIXME "internal" should expose fields too so that even parameters are readable
* FIXME use a builder pattern or something like that to avoid the Nones in oneof enums
* FIXME maybe there's a way to get rid of &Try(From) duplicates which accept a reference
