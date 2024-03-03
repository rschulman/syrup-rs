use syrup_rs::from_str;

#[cfg(test)]
mod tests {
    use preserves::value::{IOValue, NestedValue};

    #[test]
    fn test_boolean() {
        let mut input = "t";
        let response = crate::from_str(&mut input).expect("Failed to parse boolean test");

        assert_eq!(
            true,
            response
                .value_owned()
                .to_boolean()
                .expect("IOValue was not a boolean")
        );

        let mut second_input = "f";
        let second_response =
            crate::from_str(&mut second_input).expect("Failed to parse boolean test");

        assert_eq!(
            false,
            second_response
                .value_owned()
                .to_boolean()
                .expect("IOValue was not a boolean")
        );
    }

    #[test]
    fn test_positive_integer() {
        let mut input = "42+";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");

        assert_eq!(
            42,
            response
                .value_owned()
                .to_signedinteger()
                .expect("IOValue was not an integer")
                .try_into()
                .expect("Couldn't cast IOValue to i32")
        );
    }

    #[test]
    fn test_negative_integer() {
        let mut input = "756-";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");

        assert_eq!(
            -756,
            response
                .value_owned()
                .to_signedinteger()
                .expect("IOValue was not an integer")
                .try_into()
                .expect("Couldn't cast IOValue to i32")
        );
    }

    #[test]
    fn test_bytestring() {
        let mut input = "12:a bytestring";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");
        let expected: Vec<u8> = vec![
            0x61, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
        ];
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_bytestring()
                .expect("IOValue was not a bytestring")
        );
    }

    #[test]
    fn test_string() {
        let mut input = "8\"a string";
        let response = crate::from_str(&mut input).expect("Failed to parse string test");
        assert_eq!(
            "a string".to_owned(),
            response
                .value_owned()
                .into_string()
                .expect("IOValue was not a string")
        );
    }

    #[test]
    fn test_symbol() {
        let mut input = "3'foo";
        let response = crate::from_str(&mut input).expect("Failed to parse symbol test");
        assert_eq!(
            "foo".to_owned(),
            response
                .value_owned()
                .into_symbol()
                .expect("IOValue was not a symbol")
        );
    }

    #[test]
    fn test_dictionary() {
        let mut input = "{4\"name3\"bob3\"age12+8\"favorite5\"pizza}";
        let response = crate::from_str(&mut input).expect("Failed to parse dictionary test");
        let mut expected = preserves::value::Map::new();
        expected.insert(IOValue::new("name"), IOValue::new("bob"));
        expected.insert(IOValue::new("age"), IOValue::new(12));
        expected.insert(IOValue::new("favorite"), IOValue::new("pizza"));
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_dictionary()
                .expect("IOValue was not a dictionary")
        );
    }

    #[test]
    fn test_list() {
        let mut input = "[4\"name3\"bob3\"age12+8\"favorite5\"pizza]";
        let response = crate::from_str(&mut input).expect("Failed to parse dictionary test");
        let mut expected = Vec::new();
        expected.push(IOValue::new("name"));
        expected.push(IOValue::new("bob"));
        expected.push(IOValue::new("age"));
        expected.push(IOValue::new(12));
        expected.push(IOValue::new("favorite"));
        expected.push(IOValue::new("pizza"));
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_sequence()
                .expect("IOValue was not a sequence")
        );
    }

    #[test]
    fn test_record() {
        let mut input = "<4\"recd3\"bob3\"age12+8\"favorite5\"pizza>";
        let response = crate::from_str(&mut input).expect("Failed to parse dictionary test");
        let mut expected = preserves::value::Value::record(IOValue::new("recd"), 5);
        let expected_fields = expected.fields_vec_mut();
        expected_fields.push(IOValue::new("bob"));
        expected_fields.push(IOValue::new("age"));
        expected_fields.push(IOValue::new(12));
        expected_fields.push(IOValue::new("favorite"));
        expected_fields.push(IOValue::new("pizza"));
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_record()
                .expect("IOValue was not a record")
        );
    }
    #[test]
    fn test_set() {
        let mut input = "#3\"set3\"bob3\"age12+8\"favorite5\"pizza$";
        let response = crate::from_str(&mut input).expect("Failed to parse dictionary test");
        let mut expected = preserves::value::Set::new();
        expected.insert(IOValue::new("set"));
        expected.insert(IOValue::new("bob"));
        expected.insert(IOValue::new("age"));
        expected.insert(IOValue::new(12));
        expected.insert(IOValue::new("favorite"));
        expected.insert(IOValue::new("pizza"));
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_set()
                .expect("IOValue was not a set")
        );
    }

    #[test]
    fn test_zoo_no_float() {
        use preserves::value::Map;
        use std::io::Read;
        let mut zoo = std::fs::File::open("tests/zoo_no_float.bin")
            .expect("Couldn't open zoo_no_float.bin test file");
        let mut zoo_str = String::new();
        zoo.read_to_string(&mut zoo_str)
            .expect("Couldn't read zoo.bin to string");

        let result = crate::from_str(&mut zoo_str.as_str());
        match result{
            Err(_) => panic!("Failed to parse zoo. Remainder:\n{}", zoo_str),
            _ => ()
        } 
        
        let mut expected = preserves::value::Value::record(IOValue::new("zoo".as_bytes()), 2);
        let expected_fields = expected.fields_vec_mut();
        expected_fields.push(IOValue::new("The Grand Menagerie"));
        let mut animal1 = Map::new();
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("age")), IOValue::new(12));
        let mut eats1 = preserves::value::Set::new();
        eats1.insert(IOValue::new("fish".as_bytes()));
        eats1.insert(IOValue::new("mice".as_bytes()));
        eats1.insert(IOValue::new("kibble".as_bytes()));
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("eats")), IOValue::new(eats1));
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("name")), IOValue::new("Tabatha"));
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("alive?")), IOValue::new(true));
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("weight")), IOValue::new(12));
        animal1.insert(IOValue::new(preserves::value::repr::Value::symbol("species")), IOValue::new("cat".as_bytes()));
        let mut animal2 = Map::new();
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("age")), IOValue::new(6));
        let mut eats2 = preserves::value::Set::new();
        eats2.insert(IOValue::new("bananas".as_bytes()));
        eats2.insert(IOValue::new("insects".as_bytes()));
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("eats")), IOValue::new(eats2));
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("name")), IOValue::new("George"));
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("alive?")), IOValue::new(false));
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("weight")), IOValue::new(43));
        animal2.insert(IOValue::new(preserves::value::repr::Value::symbol("species")), IOValue::new("monkey".as_bytes()));
        let mut animal3 = Map::new();
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("age")), IOValue::new(-12));
        let eats3 = preserves::value::Set::new();
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("eats")), IOValue::new(eats3));
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("name")), IOValue::new("Casper"));
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("alive?")), IOValue::new(false));
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("weight")), IOValue::new(1));
        animal3.insert(IOValue::new(preserves::value::repr::Value::symbol("species")), IOValue::new("ghost".as_bytes()));
        expected_fields.push(IOValue::new(vec![
            IOValue::new(animal1),
            IOValue::new(animal2),
            IOValue::new(animal3),
        ]));

        assert_eq!(result.unwrap(), expected.finish().wrap());
    }
}
