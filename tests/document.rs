#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use arangors::{
    document::{
        DocumentInsertOptions, DocumentOverwriteMode, DocumentReadOptions, DocumentRemoveOptions,
        DocumentReplaceOptions, DocumentResponse, DocumentUpdateOptions,
    },
    ClientError, Connection, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};
use std::{convert::TryInto, ptr::null};

pub mod common;

#[cfg(not(feature = "arango3_7"))]
#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_post_create_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_create_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true, "drop collection");
    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"Trying to make unit test for createDocument but there are many cases to handle"
    }));

    // First test is to create a simple document without options
    let create = coll.create_document(test_doc, Default::default()).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");
    let result = create.unwrap();

    assert_eq!(result.is_silent(), false);
    assert_eq!(result.has_response(), true);

    let header = result.get_response().unwrap().header;
    assert_eq!(
        header._id.is_empty(),
        false,
        "We should get the id of the document"
    );
    assert_eq!(
        header._rev.is_empty(),
        false,
        "We should get the revision of the document"
    );
    assert_eq!(
        header._key.is_empty(),
        false,
        "We should get the key of the document"
    );
    // Second test is to create a simple document with option to get the new
    // document back
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with new"
    }));

    let create = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().return_new(true).build(),
        )
        .await;
    assert_eq!(create.is_ok(), true, "succeed create a document");
    let result = create.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(
        response.new.is_some(),
        true,
        "We should get the new document under the 'new' property"
    );

    let doc = response.new.unwrap();

    assert_eq!(doc.document["testDescription"], "Test with new");

    let header = response.header;
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    let key = header._key;
    // Third test is to update a simple document with option return old
    // Should not return  anything according to doc if overWriteMode is not used for
    // now TODO update this test with overwriteMode later
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with old"
    }));
    let update = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder()
                .return_old(true)
                .overwrite(true)
                .build(),
        )
        .await;
    assert_eq!(update.is_ok(), true, "succeed update a document");
    let result = update.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(response.old.is_some(), true);

    let old_doc = response.old.unwrap();
    assert_eq!(
        old_doc.document["testDescription"], "Test with new",
        "We should get the old document under the 'old' property"
    );

    let header = response.header;

    assert_eq!(header._id.is_empty(), false,);
    assert_eq!(header._rev.is_empty(), false,);
    assert_eq!(header._key.is_empty(), false,);

    // Fourth testis about the silent option
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with silent"
    }));
    let create = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().silent(true).build(),
        )
        .await;

    assert_eq!(create.is_ok(), true, "succeed create a document silently");

    let result = create.unwrap();

    assert_eq!(result.is_silent(), true);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}

/// TODO need to use CI to validate this test
#[cfg(any(feature = "arango3_7"))]
#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_post_create_document_3_7() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_create_document_3_7";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"Trying to make unit test for createDocument but there are many cases to handle"
    }));

    // First test is to create a simple document without options
    let create = coll.create_document(test_doc, Default::default()).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");

    let result = create.unwrap();
    let response = result.get_response().unwrap();
    let header = response.header;
    assert_eq!(
        header._id.is_empty(),
        false,
        "We should get the id of the document"
    );
    assert_eq!(
        header._rev.is_empty(),
        false,
        "We should get the revision of the document"
    );
    assert_eq!(
        header._key.is_empty(),
        false,
        "We should get the key of the document"
    );
    // Second test is to create a simple document with option to get the new
    // document back
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with new"
    }));

    let create = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().return_new(true).build(),
        )
        .await;
    assert_eq!(create.is_ok(), true, "succeed create a document");
    let result = create.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(
        response.new.is_some(),
        true,
        "we should get the new document under 'new' property"
    );

    let doc = response.new.unwrap();

    assert_eq!(doc.document["testDescription"], "Test with new");

    let header = response.header;
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    let key = header._key;
    // Third test is to update a simple document with option return old
    // Should not return  anything according to doc if overWriteMode is not used for
    // now TODO update this test with overwriteMode later
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with old"
    }));
    let update = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder()
                .return_old(true)
                .overwrite(true)
                .build(),
        )
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(response.old.is_some(), true);

    let old_doc = response.old.unwrap();
    assert_eq!(
        old_doc.document["testDescription"], "Test with new",
        "We should get the old document under the 'old' property"
    );

    let header = response.header;
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    // Fourth testis about the silent option
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with silent"
    }));
    let create = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().silent(true).build(),
        )
        .await;

    let result = create.unwrap();

    assert_eq!(
        result.is_silent(),
        true,
        "silent mode should not return old document"
    );
    // Fifth test is about the overwrite _mode option ignore
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with overwrite mode"
    }));
    let update = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder()
                .return_new(true)
                .overwrite_mode(DocumentOverwriteMode::Ignore),
        )
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();
    assert_eq!(response.old.is_none(), true);
    assert_eq!(response.new.is_none(), true);
    assert_eq!(response.header.is_none(), true);

    // Sixth test is about the overwrite _mode option replace
    let test_doc: Document<Value> = Document::new(json!({ "no":3 ,
    "_key" : key,
    "testDescription":"Test with overwrite mode"
    }));
    let update = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().overwrite_mode(DocumentOverwriteMode::Replace),
        )
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();
    assert_eq!(response.old.is_none(), true);
    assert_eq!(
        response.new.is_none(),
        false,
        "we should get the new document when we use the overwriteMode = 'replace'"
    );

    let doc = response.new.unwrap();
    assert_eq!(doc.document["no"], 3);

    assert_eq!(response.header.is_none(), false);
    // Seventh test is about the overwrite _mode option update
    let test_doc: Document<Value> = Document::new(json!({ "no":4 ,
    "_key" : key,
    }));
    let update = coll
        .create_document(
            test_doc,
            DocumentInsertOptions::builder().overwrite_mode(DocumentOverwriteMode::Update),
        )
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(response.old.is_none(), true);
    assert_eq!(response.new.is_none(), false);

    let doc = response.new.unwrap();
    assert_eq!(doc.document["no"], 4);

    assert_eq!(response.header.is_none(), false);

    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_read_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_read_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"read a document"
    }));

    // First test is to read a simple document without options
    let create = coll.create_document(test_doc, Default::default()).await;
    assert_eq!(create.is_ok(), true, "succeed create a document");

    let header = create.unwrap().get_response().unwrap().header;
    let _key = header._key;
    let _rev = header._rev;
    let read = coll.read_document(_key.as_str()).await;

    let result: Document<Value> = read.unwrap();

    assert_eq!(result.document["no"], 1);
    assert_eq!(result.document["testDescription"], "read a document");
    // Test if we get the right doc when it does match
    let read: Result<Document<Value>, ClientError> = coll
        .read_document_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch(_rev.clone())),
        )
        .await;
    assert_eq!(read.is_err(), false, "got the right document");
    // Test if we get the 412 code response when there is no match
    let read: Result<Document<Value>, ClientError> = coll
        .read_document_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch("_dsdsds_d".to_string())),
        )
        .await;
    // We should get a 412, for now for some reason the error is parsed as a
    // document todo fix how the reponse/error is built
    assert_eq!(
        read.is_err(),
        true,
        "we should get 412, got: {:?}",
        read.unwrap().document
    );

    // todo need to test with with IfNoneMatch and 304

    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_read_document_header() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_read_document_header";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true, "drop collection");
    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");
    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"read a document"
    }));

    // First test is to read a simple document without options
    let create = coll.create_document(test_doc, Default::default()).await;
    assert_eq!(create.is_ok(), true, "succeed create a document");

    let resp = create.unwrap().get_response().unwrap();
    let header = resp.header;
    let _key = header._key;
    let _rev = header._rev;

    let read = coll.read_document_header(_key.as_str()).await;

    assert_eq!(
        read.is_ok(),
        true,
        "We should get 200, got {:?}",
        read.err().unwrap()
    );

    let result = read.unwrap();
    assert_eq!(
        result._key, _key,
        "We should got the key of the document  : {:?}",
        result._key
    );

    let read = coll
        .read_document_header_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch(_rev.clone())),
        )
        .await;

    assert_eq!(read.is_ok(), true, "We should have the right header");

    let result = read.unwrap();
    assert_eq!(result._key, _key,);

    let read = coll
        .read_document_header_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch("_dsdsds".to_string())),
        )
        .await;

    assert_eq!(
        read.is_err(),
        true,
        "We should have an error and the right doc returned"
    );
    let read = coll
        .read_document_header_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfNoneMatch(_rev.clone())),
        )
        .await;

    assert_eq!(
        read.is_err(),
        true,
        "the If-None-Match header is given and the document has the same version"
    );
    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_patch_update_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_update_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true, "drop collection");
    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));

    // First test is to update a simple document without options
    let create = coll.create_document(test_doc, Default::default()).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");

    let _key = create.unwrap().get_response().unwrap().header._key;

    let update = coll
        .update_document(
            _key.as_str(),
            json!({ "no":2}),
            Some(
                DocumentUpdateOptions::builder()
                    .return_new(true)
                    .return_old(true)
                    .build(),
            ),
        )
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();
    let new_doc = response.new.unwrap();
    let old_doc = response.old.unwrap();

    assert_eq!(new_doc["no"], 2);
    assert_eq!(new_doc["testDescription"], "update document");

    assert_eq!(old_doc["no"], 1);
    assert_eq!(old_doc["testDescription"], "update document");

    let _rev = response.header._rev;
    let update = coll
        .update_document(_key.as_str(), json!({ "no":3}), None)
        .await;

    let result = update.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(
        response.header._rev != _rev,
        true,
        "We should get a different revision after update"
    );

    // Test when we do not ignore_revs. W
    let replace = coll
        .update_document(
            _key.as_str(),
            json!({ "no":2 , "_rev" :"_dsds_dsds_dsds_" }),
            Some(DocumentUpdateOptions::builder().ignore_revs(false).build()),
        )
        .await;

    assert_eq!(
        replace.is_err(),
        true,
        "We should have precondition failed as we ask to replace the doc only if for the \
         specified _rev in body"
    );

    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");
    // todo do more test for merge objects and stuff
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_post_replace_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_replace_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true, "drop collection");
    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));

    // First test is to replace  simple document with new & old options
    let create = coll.create_document(test_doc, Default::default()).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");
    let response = create.unwrap().get_response().unwrap();
    let header = response.header;
    let _key = header._key;
    let _rev = header._rev;

    let replace = coll
        .replace_document(
            _key.as_str(),
            json!({ "no":2}),
            Some(
                DocumentReplaceOptions::builder()
                    .return_new(true)
                    .return_old(true)
                    .build(),
            ),
            None,
        )
        .await;

    let result = replace.unwrap();
    let response = result.get_response().unwrap();
    let new_doc: Value = response.new.unwrap();

    assert_eq!(new_doc["no"], 2, "We should get the property updated");
    assert_eq!(
        new_doc["testDescription"].as_str().is_some(),
        false,
        "We should get the property removed sience we did replace the original object with an \
         object that do not have it"
    );

    let old_doc: Value = response.old.unwrap();

    assert_eq!(
        old_doc["no"], 1,
        "We should get the old property no with its old value"
    );
    assert_eq!(
        old_doc["testDescription"], "update document",
        "We should get the old property testDescription with its old value"
    );

    // Second test to try out the silence mode

    let replace = coll
        .replace_document(
            _key.as_str(),
            json!({ "no":2}),
            Some(DocumentReplaceOptions::builder().silent(true).build()),
            None,
        )
        .await;

    let result = replace.unwrap();
    assert_eq!(result.is_silent(), true, "We should not get any response");

    // third test tro try out the if-match header

    let replace = coll
        .replace_document(_key.as_str(), json!({ "no":2}), None, Some(_rev.clone()))
        .await;

    assert_eq!(
        replace.is_err(),
        true,
        "We should have precondition failed as we ask to replace the doc only if match the \
         specified _rev in header"
    );

    let replace = coll
        .replace_document(
            _key.as_str(),
            json!({ "no":2 , "_rev" :_rev.clone() }),
            Some(DocumentReplaceOptions::builder().ignore_revs(false).build()),
            None,
        )
        .await;

    assert_eq!(
        replace.is_err(),
        true,
        "We should have precondition failed as we ask to replace the doc only if match the \
         specified _rev in body"
    );

    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");

    // todo do more test
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_delete_remove_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_remove_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true, "drop collection");
    let coll = database.create_collection(collection_name).await;
    coll.expect("Should create the collection");

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));

    // First test is to remove a simple document with old options
    let create: Result<DocumentResponse<Document<Value>>, ClientError> =
        coll.create_document(test_doc, Default::default()).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");
    let response = create.unwrap().get_response().unwrap();
    let header = response.header;
    let _key = header._key;
    let _rev = header._rev;

    let remove: Result<DocumentResponse<Value>, ClientError> = coll
        .remove_document(
            _key.as_str(),
            Some(DocumentRemoveOptions::builder().return_old(true).build()),
            None,
        )
        .await;

    let result = remove.unwrap();
    let response = result.get_response().unwrap();

    assert_eq!(
        response.new.is_none(),
        true,
        "we should never have new doc returned when using remove document"
    );

    let old_doc: Value = response.old.unwrap();

    assert_eq!(
        old_doc["no"], 1,
        "We should get the old property no with its old value"
    );
    assert_eq!(
        old_doc["testDescription"], "update document",
        "We should get the old property testDescription with its old value"
    );

    // Second test to try out the silence mode
    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));
    let create = coll.create_document(test_doc, Default::default()).await;
    let response = create.unwrap().get_response().unwrap();
    let header = response.header;
    let _key = header._key;
    let _rev = header._rev;
    let remove: Result<DocumentResponse<Value>, ClientError> = coll
        .remove_document(
            _key.as_str(),
            Some(DocumentRemoveOptions::builder().silent(true).build()),
            None,
        )
        .await;

    let result = remove.unwrap();

    assert_eq!(result.is_silent(), true, "We should not get any response");

    // third test to try out the If-Match header
    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));
    let create = coll.create_document(test_doc, Default::default()).await;
    let response = create.unwrap().get_response().unwrap();
    let header = response.header;
    let _key = header._key;
    let _rev = header._rev;
    let remove: Result<DocumentResponse<Value>, ClientError> = coll
        .remove_document(_key.as_str(), None, Some("_rere_dsds_DSds".to_string()))
        .await;

    assert_eq!(
        remove.is_err(),
        true,
        "We should have precondition failed as we ask to move the doc only if for the specified \
         _rev in header"
    );
    // Fourth test to check that we get error if we tried to remove a doc that has
    // already been removed or that does not exist
    let remove: Result<DocumentResponse<Value>, ClientError> =
        coll.remove_document(_key.as_str(), None, None).await;

    assert_eq!(remove.is_err(), false, "We should remove the doc");

    let remove: Result<DocumentResponse<Value>, ClientError> =
        coll.remove_document(_key.as_str(), None, None).await;

    assert_eq!(
        remove.is_err(),
        true,
        "We should get 404 because we just have removed the doc before"
    );

    let coll = database.drop_collection(collection_name).await;
    coll.expect("Should drop the collection");
    // todo do more test
}
