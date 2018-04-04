#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation_and_insertion() {

        let mut node = Node::new("hello");

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hello"), true);

        assert_eq!(node.exists("hellowor"), false);

        const INSERTED_CHARACTERS: &str = "helloworld";
        node.insert(INSERTED_CHARACTERS);

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), INSERTED_CHARACTERS);

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hellowor"), true);
        assert_eq!(node.exists("helloworld"), true);

        assert_eq!(node.exists("helloworldandmore"), false);

        assert!(children[0].get_children().is_empty());
    }

    #[test]
    fn test_create_children_with_insertion_longer_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bonjour";

        let mut node = Node::new(FIRST_CHARACTERS);

        {
            let children = node.get_children();
            assert!(children[0].get_children().is_empty());
        }

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        assert_eq!(node.get_children().len(), 2);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);

        assert!(children[0].get_children().is_empty());
        assert!(children[1].get_children().is_empty());
    }

    #[test]
    fn test_create_children_with_insertion_shorter_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bon";

        let mut node = Node::new(FIRST_CHARACTERS);

        {
            let children = node.get_children();
            assert!(children[0].get_children().is_empty());
        }

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        assert_eq!(node.get_children().len(), 2);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        let children = node.get_children();
        assert_eq!(node.get_characters(), "");
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);

        assert!(children[0].get_children().is_empty());
        assert!(children[1].get_children().is_empty());
    }

    #[test]
    fn test_get_children_from_root_has_one_child_by_default() {

        let node = Node::new("hello");
        assert_eq!(node.get_children().len(), 1);
    }

    #[test]
    fn test_characters_exist_into_root_node() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().len(), 1);

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hella"), false);
        assert_eq!(node.exists("bonjour"), false);

        let children = node.get_children();
        assert!(children[0].get_children().is_empty());
    }

    #[test]
    fn test_characters_exist_into_root_node_with_small_word() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().len(), 1);

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hey"), false);

        let children = node.get_children();
        assert!(children[0].get_children().is_empty());
    }

    #[test]
    fn test_characters_exist_into_root_node_with_long_word() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().len(), 1);

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("helloworld"), false);

        let children = node.get_children();
        assert!(children[0].get_children().is_empty());
    }

    #[test]
    fn test_split_characters_at_the_beginning() {

        let mut node = Node::new("bonjour");
        node.insert("bien");

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), "b");

        let sub_children = children[0].get_children();

        assert_eq!(sub_children.len(), 2);

        assert_eq!(sub_children[0].get_characters(), "onjour");
        assert_eq!(sub_children[1].get_characters(), "ien");

        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
    }

    #[test]
    fn test_split_characters_in_the_middle() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "bon");
        assert_eq!(sub_children[0].get_characters(), "jour");
        assert_eq!(sub_children[1].get_characters(), "app");

        assert_eq!(sub_children.len(), 2);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
    }

    #[test]
    fn test_split_characters_at_the_end() {

        let mut node = Node::new("bona");
        node.insert("boni");

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "bon");
        assert_eq!(sub_children[0].get_characters(), "a");
        assert_eq!(sub_children[1].get_characters(), "i");

        assert_eq!(sub_children.len(), 2);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
    }

    #[test]
    fn test_add_three_children_to_node() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");
        node.insert("bonsoir");

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "bon");
        assert_eq!(sub_children[0].get_characters(), "jour");
        assert_eq!(sub_children[1].get_characters(), "app");
        assert_eq!(sub_children[2].get_characters(), "soir");

        assert_eq!(node.exists("b"), true);
        assert_eq!(node.exists("bon"), true);
        assert_eq!(node.exists("bonap"), true);
        assert_eq!(node.exists("bonjour"), true);
        assert_eq!(node.exists("bonapp"), true);
        assert_eq!(node.exists("bonsoir"), true);

        assert_eq!(node.exists("a"), false);
        assert_eq!(node.exists("boa"), false);
        assert_eq!(node.exists("bonaa"), false);
        assert_eq!(node.exists("bonjoua"), false);
        assert_eq!(node.exists("bonapi"), false);
        assert_eq!(node.exists("hello"), false);

        assert_eq!(sub_children.len(), 3);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
        assert!(sub_children[2].get_children().is_empty());
    }

    #[test]
    fn test_add_four_children_to_node() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");
        node.insert("bonsoir");
        node.insert("bonnenuit");

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "bon");
        assert_eq!(sub_children[0].get_characters(), "jour");
        assert_eq!(sub_children[1].get_characters(), "app");
        assert_eq!(sub_children[2].get_characters(), "soir");
        assert_eq!(sub_children[3].get_characters(), "nenuit");

        assert_eq!(node.exists("bo"), true);
        assert_eq!(node.exists("bon"), true);
        assert_eq!(node.exists("bons"), true);
        assert_eq!(node.exists("bonnen"), true);
        assert_eq!(node.exists("bonjour"), true);
        assert_eq!(node.exists("bonapp"), true);
        assert_eq!(node.exists("bonsoir"), true);
        assert_eq!(node.exists("bonnenuit"), true);

        assert_eq!(node.exists("ba"), false);
        assert_eq!(node.exists("boa"), false);
        assert_eq!(node.exists("boni"), false);
        assert_eq!(node.exists("bonnea"), false);
        assert_eq!(node.exists("bonsour"), false);
        assert_eq!(node.exists("bonappet"), false);
        assert_eq!(node.exists("hello"), false);

        assert_eq!(sub_children.len(), 4);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
        assert!(sub_children[2].get_children().is_empty());
        assert!(sub_children[3].get_children().is_empty());
    }

    #[test]
    fn test_add_three_children_to_node_with_different_root() {

        let mut node = Node::new("jour");
        node.insert("app");
        node.insert("soir");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 3);

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
        assert_eq!(children[2].get_characters(), "soir");

        assert_eq!(node.exists("jour"), true);
        assert_eq!(node.exists("app"), true);
        assert_eq!(node.exists("soir"), true);

        assert_eq!(node.exists("journee"), false);
        assert_eq!(node.exists(" app"), false);
        assert_eq!(node.exists("siir"), false);
    }

    #[test]
    fn test_add_four_children_to_node_with_different_root() {

        let mut node = Node::new("jour");
        node.insert("app");
        node.insert("soir");
        node.insert("neapp");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 4);

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
        assert_eq!(children[2].get_characters(), "soir");
        assert_eq!(children[3].get_characters(), "neapp");

        assert_eq!(node.exists("jour"), true);
        assert_eq!(node.exists("app"), true);
        assert_eq!(node.exists("soir"), true);
        assert_eq!(node.exists("neapp"), true);

        assert_eq!(node.exists("journee"), false);
        assert_eq!(node.exists(" app"), false);
        assert_eq!(node.exists("siir"), false);
        assert_eq!(node.exists("lol"), false);
    }

    #[test]
    fn test_update_first_child_characters() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[1].get_characters(), "me");

            assert_eq!(children[0].get_children().len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }

        node.insert("salted");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lted");

            assert_eq!(children[0].get_children().len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }

        assert_eq!(node.exists("s"), true);
        assert_eq!(node.exists("sal"), true);
        assert_eq!(node.exists("salt"), true);
        assert_eq!(node.exists("salte"), true);
        assert_eq!(node.exists("salted"), true);
        assert_eq!(node.exists("sam"), true);
        assert_eq!(node.exists("same"), true);

        assert_eq!(node.exists("u"), false);
        assert_eq!(node.exists("sul"), false);
        assert_eq!(node.exists("salu"), false);
        assert_eq!(node.exists("saltu"), false);
        assert_eq!(node.exists("salteu"), false);
        assert_eq!(node.exists("sau"), false);
        assert_eq!(node.exists("samu"), false);
        assert_eq!(node.exists("samed"), false);
    }

    #[test]
    fn test_update_second_child_characters() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[1].get_characters(), "me");

            assert_eq!(children[0].get_children().len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }

        node.insert("sameless");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[1].get_characters(), "meless");

            assert_eq!(children[0].get_children().len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }
    }

    #[test]
    fn test_characters_exist_into_root_node_when_single_character_children_exist() {

        let mut node = Node::new("hello");
        node.insert("hella");

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hella"), true);

        assert_eq!(node.exists("helli"), false);
        assert_eq!(node.exists("hellooop"), false);
        assert_eq!(node.exists("helliot"), false);

        assert_eq!(node.exists("he"), true);

        assert_eq!(node.exists("hey"), false);

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "hell");
        assert_eq!(sub_children[0].get_characters(), "o");
        assert_eq!(sub_children[1].get_characters(), "a");

        assert_eq!(children[0].get_children().len(), 2);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
    }

    #[test]
    fn test_characters_exist_into_root_node_when_multiple_children_children_exist() {

        let mut node = Node::new("helloworld");
        node.insert("helloearth");

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hella"), false);

        assert_eq!(node.exists("helloworld"), true);
        assert_eq!(node.exists("helloearth"), true);
        assert_eq!(node.exists("hellowo"), true);
        assert_eq!(node.exists("helloea"), true);

        assert_eq!(node.exists("hellowooow"), false);
        assert_eq!(node.exists("helloearti"), false);
        assert_eq!(node.exists("hellowa"), false);
        assert_eq!(node.exists("helloei"), false);

        assert_eq!(node.exists("helloworldandmore"), false);
        assert_eq!(node.exists("helloearthandmore"), false);

        let children = node.get_children();
        let sub_children = children[0].get_children();

        assert_eq!(children[0].get_characters(), "hello");
        assert_eq!(sub_children[0].get_characters(), "world");
        assert_eq!(sub_children[1].get_characters(), "earth");

        assert_eq!(children[0].get_children().len(), 2);
        assert!(sub_children[0].get_children().is_empty());
        assert!(sub_children[1].get_children().is_empty());
    }

    #[test]
    fn test_create_two_subchildren() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[1].get_characters(), "me");

            assert_eq!(sub_children.len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }

        node.insert("salted");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(sub_children[0].get_characters(), "lted");
            assert_eq!(sub_children[1].get_characters(), "me");

            assert_eq!(sub_children.len(), 2);
            assert!(sub_children[0].get_children().is_empty());
            assert!(sub_children[1].get_children().is_empty());
        }

        node.insert("saltandpepper");

        let children = node.get_children();
        let sub_children = children[0].get_children();
        let sub_sub_children = sub_children[0].get_children();

        assert_eq!(children[0].get_characters(), "sa");
        assert_eq!(sub_children[0].get_characters(), "lt");
        assert_eq!(sub_sub_children[0].get_characters(), "ed");
        assert_eq!(sub_sub_children[1].get_characters(), "andpepper");

        assert!(sub_children[1].get_children().is_empty());
        assert!(sub_sub_children[0].get_children().is_empty());
        assert!(sub_sub_children[1].get_children().is_empty());

        assert_eq!(node.exists("sal"), true);
        assert_eq!(node.exists("salt"), true);
        assert_eq!(node.exists("saltand"), true);
        assert_eq!(node.exists("saltandpepper"), true);
        assert_eq!(node.exists("salte"), true);
        assert_eq!(node.exists("salted"), true);
        assert_eq!(node.exists("sam"), true);
        assert_eq!(node.exists("same"), true);

        assert_eq!(node.exists("sao"), false);
        assert_eq!(node.exists("sali"), false);
        assert_eq!(node.exists("saltani"), false);
        assert_eq!(node.exists("saltandpeppor"), false);
        assert_eq!(node.exists("salti"), false);
        assert_eq!(node.exists("salter"), false);
        assert_eq!(node.exists("sao"), false);
        assert_eq!(node.exists("sami"), false);
    }

    #[test]
    fn test_create_three_subchildren() {

        let mut node = Node::new("salt");
        node.insert("same");
        node.insert("salted");
        node.insert("saltandpepper");
        node.insert("salto");

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "sa");
        assert_eq!(children[0].get_children().len(), 2);

        let sub_children = children[0].get_children();

        assert_eq!(sub_children[0].get_characters(), "lt");
        assert_eq!(sub_children[0].get_children().len(), 3);

        assert_eq!(sub_children[1].get_characters(), "me");
        assert!(sub_children[1].get_children().is_empty());

        let first_child_subchildren = sub_children[0].get_children();

        assert_eq!(first_child_subchildren[0].get_characters(), "ed");
        assert_eq!(first_child_subchildren[1].get_characters(), "andpepper");
        assert_eq!(first_child_subchildren[2].get_characters(), "o");

        assert!(first_child_subchildren[0].get_children().is_empty());
        assert!(first_child_subchildren[1].get_children().is_empty());
        assert!(first_child_subchildren[2].get_children().is_empty());

        assert_eq!(node.exists("salt"), true);
        assert_eq!(node.exists("same"), true);
        assert_eq!(node.exists("salted"), true);
        assert_eq!(node.exists("saltandpepper"), true);
        assert_eq!(node.exists("salto"), true);
    }

    #[test]
    fn test_create_two_subsubchildren() {

        let mut node = Node::new("salt");
        node.insert("same");
        node.insert("salted");
        node.insert("saltandpepper");
        node.insert("saltandketchup");

        assert_eq!(node.get_characters(), "");

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "sa");
        assert_eq!(children[0].get_children().len(), 2);

        let sub_children = children[0].get_children();

        assert_eq!(sub_children[0].get_characters(), "lt");
        assert_eq!(sub_children[0].get_children().len(), 2);

        assert_eq!(sub_children[1].get_characters(), "me");
        assert!(sub_children[1].get_children().is_empty());

        let sub_children = sub_children[0].get_children();

        assert_eq!(sub_children[0].get_characters(), "ed");
        assert!(sub_children[0].get_children().is_empty());

        assert_eq!(sub_children[1].get_characters(), "and");
        assert_eq!(sub_children[1].get_children().len(), 2);

        let subsub_children = sub_children[1].get_children();

        assert_eq!(subsub_children[0].get_characters(), "pepper");
        assert!(subsub_children[0].get_children().is_empty());

        assert_eq!(subsub_children[1].get_characters(), "ketchup");
        assert!(subsub_children[1].get_children().is_empty());

        assert_eq!(node.exists("salt"), true);
        assert_eq!(node.exists("same"), true);
        assert_eq!(node.exists("salted"), true);
        assert_eq!(node.exists("saltandpepper"), true);
        assert_eq!(node.exists("saltandketc"), true);
        assert_eq!(node.exists("saltandketchup"), true);
    }

    #[test]
    fn test_add_root_node_child_when_child_and_subchildren_exists() {

        let mut node = Node::new("salt");
        node.insert("same");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 1);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "lt");
            assert!(sub_children[0].get_children().is_empty());

            assert_eq!(sub_children[1].get_characters(), "me");
            assert!(sub_children[1].get_children().is_empty());
        }

        node.insert("hello");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 2);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            assert_eq!(children[1].get_characters(), "hello");
            assert!(children[1].get_children().is_empty());
        }

        node.insert("bonjour");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 3);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            assert_eq!(children[1].get_characters(), "hello");
            assert!(children[1].get_children().is_empty());

            assert_eq!(children[2].get_characters(), "bonjour");
            assert!(children[2].get_children().is_empty());
        }

        node.insert("hella");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 3);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            assert_eq!(children[1].get_characters(), "hell");
            assert_eq!(children[1].get_children().len(), 2);

            assert_eq!(children[2].get_characters(), "bonjour");
            assert!(children[2].get_children().is_empty());

            let sub_children = children[1].get_children();

            assert_eq!(sub_children[0].get_characters(), "o");
            assert!(sub_children[0].get_children().is_empty());

            assert_eq!(sub_children[1].get_characters(), "a");
            assert!(sub_children[1].get_children().is_empty());
        }
    }

    #[test]
    fn test_add_root_child_node_child_when_child_and_subchildren_exists() {

        let mut node = Node::new("sa");
        node.insert("hello");
        node.insert("same");
        node.insert("salt");
        node.insert("saltandpepper");
        node.insert("salted");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 2);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            assert_eq!(children[1].get_characters(), "hello");
            assert!(children[1].get_children().is_empty());

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "me");
            assert!(sub_children[0].get_children().is_empty());

            assert_eq!(sub_children[1].get_characters(), "lt");
            assert_eq!(sub_children[1].get_children().len(), 2);

            let sub_sub_children = sub_children[1].get_children();

            assert_eq!(sub_sub_children[0].get_characters(), "andpepper");
            assert!(sub_sub_children[0].get_children().is_empty());

            assert_eq!(sub_sub_children[1].get_characters(), "ed");
            assert!(sub_sub_children[1].get_children().is_empty());
        }

        node.insert("sad");

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "sa");
        assert_eq!(children[0].get_children().len(), 3);

        assert_eq!(children[1].get_characters(), "hello");
        assert!(children[1].get_children().is_empty());

        let sub_children = children[0].get_children();

        assert_eq!(sub_children[0].get_characters(), "me");
        assert!(sub_children[0].get_children().is_empty());

        assert_eq!(sub_children[1].get_characters(), "lt");
        assert_eq!(sub_children[1].get_children().len(), 2);

        assert_eq!(sub_children[2].get_characters(), "d");
        assert!(sub_children[2].get_children().is_empty());
    }

    #[test]
    fn test_replace_root_child_characters() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[0].get_children().is_empty(), true);

            assert_eq!(sub_children[1].get_characters(), "me");
            assert_eq!(sub_children[1].get_children().is_empty(), true);
        }

        node.insert("sol");

        assert_eq!(node.get_characters(), "");
        assert_eq!(node.get_children().len(), 1);

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "s");
            assert_eq!(children[0].get_children().len(), 2);

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "a");
            assert_eq!(sub_children[0].get_children().len(), 2);

            assert_eq!(sub_children[1].get_characters(), "ol");
            assert!(sub_children[1].get_children().is_empty());

            let sub_sub_children = sub_children[0].get_children();

            assert_eq!(sub_sub_children[0].get_characters(), "lt");
            assert!(sub_sub_children[0].get_children().is_empty());

            assert_eq!(sub_sub_children[1].get_characters(), "me");
            assert!(sub_sub_children[1].get_children().is_empty());
        }

        assert!(node.exists("salt"));
        assert!(node.exists("same"));
        assert!(node.exists("sol"));

        assert!(node.exists("sal"));
        assert!(node.exists("sam"));
        assert!(node.exists("so"));
    }

    #[test]
    fn test_replace_root_child_characters_and_move_sub_children() {

        let mut node = Node::new("salt");
        node.insert("same");
        node.insert("salted");
        node.insert("saltandpepper");

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "sa");
            assert_eq!(children[0].get_children().len(), 2);

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "lt");
            assert_eq!(sub_children[0].get_children().len(), 2);

            assert_eq!(sub_children[1].get_characters(), "me");
            assert!(sub_children[1].get_children().is_empty());

            let sub_sub_children = sub_children[0].get_children();

            assert_eq!(sub_sub_children[0].get_characters(), "ed");
            assert!(sub_sub_children[0].get_children().is_empty());

            assert_eq!(sub_sub_children[1].get_characters(), "andpepper");
            assert!(sub_sub_children[1].get_children().is_empty());
        }

        node.insert("sol");

        {
            let children = node.get_children();

            assert_eq!(children[0].get_characters(), "s");
            assert_eq!(children[0].get_children().len(), 2);

            let sub_children = children[0].get_children();

            assert_eq!(sub_children[0].get_characters(), "a");
            assert_eq!(sub_children[0].get_children().len(), 2);

            assert_eq!(sub_children[1].get_characters(), "ol");
            assert!(sub_children[1].get_children().is_empty());

            let sub_sub_children = sub_children[0].get_children();

            assert_eq!(sub_sub_children[0].get_characters(), "lt");
            assert_eq!(sub_sub_children[0].get_children().len(), 2);

            assert_eq!(sub_sub_children[1].get_characters(), "me");
            assert!(sub_sub_children[1].get_children().is_empty());

            let sub_sub_sub_children = sub_sub_children[0].get_children();

            assert_eq!(sub_sub_sub_children[0].get_characters(), "ed");
            assert!(sub_sub_sub_children[0].get_children().is_empty());

            assert_eq!(sub_sub_sub_children[1].get_characters(), "andpepper");
            assert!(sub_sub_sub_children[1].get_children().is_empty());
        }
    }
}
