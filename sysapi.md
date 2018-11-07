## sysapi

    - "/version":
                 ✅ GET    > returns JSON object with version field.


    - "/datasets/list/:cellname":
                 ✅ GET    > returns JSON list of all datasets of given cellname.


    - "/cells/list":

                 ✅ GET    > returns JSON list of all created cells


    - "/cell/:cellname":

                 ✅ POST   > creates new cell, body contains mandatory ed25519 pubkey [root part] as String
                 ✅ GET    > returns JSON with status of the 'cellname' cell
                 ✅ DELETE > destroy cell 'cellname'


     - "/snapshot/list/:cellname"

                 ✅ GET    > list snapshots of given cell


     - "/snapshot/:cellname/:snapname":

                 ✅ POST   > creates a snapshot of dataset_path [given in body as String]
                 ✅ GET    > returns JSON with snapshot-existence-confirmation
                 ✅ DELETE > destroys snapshot


     - "/rollback/:cellname/:snapname":

                 ✅ POST   > rollbacks dataset to given snapshot [dataset_path is given in body as String]


     - "/proxy/:cellname"
                     GET    > get list of Nginx proxy for cell: 'cellname'.


     - "/proxy/:cellname/:from-domain/:to-domain"

                  ✅ POST   > defines new Nginx-Proxy entry, from external: ':from-domain', to internal: ':to-domain'.
                     GET    > returns domain to proxy to
                     DELETE > destroys domain proxy


    - "/cell/:cellname/some_key":

                    POST   > body contains new value of the key: 'some_key' to set for cell 'cellname'
                    GET    > returns value of 'some_key' for cell 'cellname'
                    DELETE > unset value of 'some_key' for cell 'cellname'.


    - "/igniter/:cellname/Redis/start"   ||
    - "/igniter/:cellname/Redis/reload"  ||
    - "/igniter/:cellname/Redis/restart" ||
    - "/igniter/:cellname/Php72/stop"    ||
    - "/igniter/:cellname/Php72/test"    ||
    - "/igniter/:cellname/Php72/watch"   ||
    - "/igniter/:cellname/Rust/install":

                    POST   > control service igniter under 'cellname' cell.


    - "/zone/:cellname"
                    GET    > get metadata about all DNS zone used by 'cellname'.


    - "/zone/:cellname/A/some.domain.local" ||
    - "/zone/:cellname/TXT/some.domain.local" ||
    - "/zone/:cellname/CNAME/my.some.domain.local":

                    POST   > defines new A record for cell 'cellname' with IPv4 or name taken from the body


    - "/status/:cellname":

                    GET    > returns JSON metadata, limits and status for all services under 'cellname' cell.


    - "/status/:cellname/Php72":

                    GET    > returns JSON metadata and stats of service Php72.

