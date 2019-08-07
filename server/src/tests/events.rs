use std::collections::HashSet;

use opcua_types::{
    Variant, NodeId, AttributeId, UAString, ObjectId, ObjectTypeId, VariableTypeId, QualifiedName, LocalizedText,
    operand::{Operand, ContentFilterBuilder},
    node_ids::ReferenceTypeId,
    service_types::ContentFilterElement,
};

use crate::{
    tests::*,
    address_space::{
        AddressSpace,
        object_type::ObjectTypeBuilder,
        variable::{Variable, VariableBuilder},
    },
    events::operator,
    events::event_filter,
    events::event::{Event, BaseEventType},
};

fn event_type_id() -> NodeId {
    NodeId::new(2, "TestEventType")
}

fn event_id() -> NodeId {
    NodeId::new(2, 1000)
}

pub struct TestEventType {
    base: BaseEventType,
    foo: i32,
}

impl Event for TestEventType {
    type Err = ();

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    fn insert<R, S, N>(self, node_id: &NodeId, browse_name: R, description: S, parent_node: N, address_space: &mut AddressSpace) -> Result<(), Self::Err>
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              N: Into<NodeId> {
        let result = self.base.insert(node_id, browse_name, description, parent_node, address_space);
        if result.is_ok() {
            Self::insert_property(node_id, 2, "Foo", "Foo", self.foo, address_space);
        }
        result
    }
}

impl TestEventType {
    pub fn new(source_object_id: &NodeId, foo: i32) -> Self {
        let mut event = Self {
            base: Default::default(),
            foo,
        };
        event.base.event_type = event_type_id();
        event.base.source_node = source_object_id.clone();
        event.base.message = LocalizedText::from(format!("A Test event from {:?}", source_object_id));
        event
    }
}

fn create_event(address_space: &mut AddressSpace, node_id: NodeId, source_machine_id: &NodeId, foo: i32) {
    let event = TestEventType::new(source_machine_id, foo);
    // create an event object in a folder with the
    let event_name = format!("Event{}", foo);
    let _ = event.insert(&node_id, event_name.clone(), event_name, NodeId::objects_folder_id(), address_space);
}

fn address_space() -> AddressSpace {
    let mut address_space = AddressSpace::new();

    // Create an event type
    let event_type_id = event_type_id();
    ObjectTypeBuilder::new(&event_type_id, "TestEventType", "TestEventType")
        .is_abstract(false)
        .subtype_of(ObjectTypeId::BaseEventType)
        .insert(&mut address_space);

    // Add attribute to event type
    let attr_foo_id = NodeId::new(2, "Foo");
    VariableBuilder::new(&attr_foo_id, "Foo", "Foo")
        .property_of(event_type_id.clone())
        .has_type_definition(VariableTypeId::PropertyType)
        .has_modelling_rule(ObjectId::ModellingRule_Mandatory)
        .insert(&mut address_space);

    // Create an event of that type
    create_event(&mut address_space, event_id(), &ObjectId::Server.into(), 100);

    address_space
}

fn do_operator_test<T>(f: T)
    where T: FnOnce(&AddressSpace, &NodeId, &mut HashSet<u32>, &Vec<ContentFilterElement>)
{
    opcua_console_logging::init();
    let mut used_elements = HashSet::new();
    let elements = vec![];
    let address_space = address_space();

    // use object_id of a generated event
    let object_id = event_id();

    f(&address_space, &object_id, &mut used_elements, &elements);
}

#[test]
fn test_eq() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = &[Operand::literal(10), Operand::literal(10)];
        let result = operator::eq(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(9), Operand::literal(10)];
        let result = operator::eq(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(10), Operand::literal(11)];
        let result = operator::eq(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_lt() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = &[Operand::literal(9), Operand::literal(10)];
        let result = operator::lt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(10), Operand::literal(10)];
        let result = operator::lt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(11), Operand::literal(10)];
        let result = operator::lt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_lte() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = &[Operand::literal(9), Operand::literal(10)];
        let result = operator::lte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(10), Operand::literal(10)];
        let result = operator::lte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(11), Operand::literal(10)];
        let result = operator::lte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_gt() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = [Operand::literal(11), Operand::literal(10)];
        let result = operator::gt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(10), Operand::literal(10)];
        let result = operator::gt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(9), Operand::literal(10)];
        let result = operator::gt(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_gte() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = &[Operand::literal(11), Operand::literal(10)];
        let result = operator::gte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(10), Operand::literal(10)];
        let result = operator::gte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(9), Operand::literal(10)];
        let result = operator::gte(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_not() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(false)];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(true)];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        // String
        let operands = &[Operand::literal("0")];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        // String(2)
        let operands = &[Operand::literal("true")];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        // Invalid - Double
        let operands = &[Operand::literal(99.9)];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        // Invalid - Int32
        let operands = &[Operand::literal(1)];
        let result = operator::not(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    });
}

#[test]
fn test_between() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        // Test operator with some ranges and mix of types with implicit conversion
        let operands = &[Operand::literal(12), Operand::literal(12), Operand::literal(13)];
        let result = operator::between(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(13), Operand::literal(12), Operand::literal(13)];
        let result = operator::between(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(12.3), Operand::literal(12.0), Operand::literal(12.4)];
        let result = operator::between(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(11.99), Operand::literal(12.0), Operand::literal(13.0)];
        let result = operator::between(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(13.0001), Operand::literal(12.0), Operand::literal(13.0)];
        let result = operator::between(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

//        let operands = &[Operand::literal("12.5"), Operand::literal(12), Operand::literal(13)]);
//        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
//        assert_eq!(result, Variant::Boolean(true));
    })
}

#[test]
fn test_and() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(true), Operand::literal(true)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(false), Operand::literal(true)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(true), Operand::literal(false)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(false), Operand::literal(false)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(true), Operand::literal(())];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = &[Operand::literal(()), Operand::literal(true)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = &[Operand::literal(false), Operand::literal(())];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(()), Operand::literal(false)];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(()), Operand::literal(())];
        let result = operator::and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    })
}

#[test]
fn test_or() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(true), Operand::literal(true)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(true), Operand::literal(false)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(false), Operand::literal(true)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(false), Operand::literal(false)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(true), Operand::literal(())];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(()), Operand::literal(true)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = &[Operand::literal(false), Operand::literal(())];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = &[Operand::literal(()), Operand::literal(false)];
        let result = operator::or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    })
}


#[test]
fn test_in_list() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(10), Operand::literal(false)];
        let result = operator::in_list(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = &[Operand::literal(true), Operand::literal(false)];
        let result = operator::in_list(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
        /*
                let operands = &[Operand::literal("true"), Operand::literal(true)];
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = &[Operand::literal(99), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)];
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = &[Operand::literal(()), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)];
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = &[Operand::literal(33), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)];
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(false));
                */
    })
}

#[test]
fn test_bitwise_or() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(0xff00u16), Operand::literal(0x00ffu16)];
        let result = operator::bitwise_or(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::UInt16(0xffff));
    })
}

#[test]
fn test_bitwise_and() {
    do_operator_test(|address_space, object_id, used_elements, elements| {
        let operands = &[Operand::literal(0xf00fu16), Operand::literal(0x00ffu16)];
        let result = operator::bitwise_and(&object_id, &operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::UInt16(0x000f));
    })
}

#[test]
fn test_where_clause() {
    let address_space = address_space();

    let object_id = NodeId::root_folder_id();

    // IsNull(NULL)
    let f = ContentFilterBuilder::new()
        .is_null(Operand::literal(()))
        .build();
    let result = event_filter::evaluate_where_clause(&object_id, &f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // (550 == "550") && (10.5 == "10.5")
    let f = ContentFilterBuilder::new()
        .and(Operand::element(1), Operand::element(2))
        .equals(Operand::literal(550), Operand::literal("550"))
        .equals(Operand::literal(10.5), Operand::literal("10.5"))
        .build();
    let result = event_filter::evaluate_where_clause(&object_id, &f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // Like operator
    let f = ContentFilterBuilder::new()
        .like(Operand::literal("Hello world"), Operand::literal("[Hh]ello w%"))
        .build();
    let result = event_filter::evaluate_where_clause(&object_id, &f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // Not equals
    let f = ContentFilterBuilder::new()
        .not(Operand::element(1))
        .equals(Operand::literal(550), Operand::literal(551))
        .build();
    let result = event_filter::evaluate_where_clause(&object_id, &f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // Do some relative path comparisons against the event to ensure content filters appear to work
    let expected = vec![
        // Valid
        (NodeId::root_folder_id(), "Objects/Event100/Foo", 100, true),
        (NodeId::objects_folder_id(), "Event100/Foo", 100, true),
        (event_id(), "Foo", 100, true),
        // Invalid
        (NodeId::root_folder_id(), "Objects/Event101/Foo", 100, false),
        (NodeId::root_folder_id(), "Objects/Foo", 100, false),
        (NodeId::root_folder_id(), "Objects/Event100/Foo", 101, false),
        (NodeId::objects_folder_id(), "Event100/Foo", 101, false),
        (event_id(), "Foo", 101, false),
        (NodeId::objects_folder_id(), "Event100/Foo/Bar", 100, false),
        (event_id(), "", 100, false),
    ];
    expected.into_iter().for_each(|(node_id, browse_path, value_to_compare, expected)| {
        let f = ContentFilterBuilder::new()
            .equals(Operand::simple_attribute(ReferenceTypeId::Organizes, browse_path, AttributeId::Value, UAString::null()), Operand::literal(value_to_compare))
            .build();
        let result = event_filter::evaluate_where_clause(&node_id, &f, &address_space);
        assert_eq!(result.unwrap(), expected.into());
    });

}