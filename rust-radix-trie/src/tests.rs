#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation_and_insertion() {

        let mut node = Node::new("hello");

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hello"), true);

        assert_eq!(node.exists("hellowor"), false);

        assert_eq!(node.get_children().is_empty(), true);

        const INSERTED_CHARACTERS: &str = "helloworld";
        node.insert(INSERTED_CHARACTERS);
        assert_eq!(node.get_characters(), INSERTED_CHARACTERS);

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hellowor"), true);
        assert_eq!(node.exists("helloworld"), true);

        assert_eq!(node.exists("helloworldandmore"), false);

        assert_eq!(node.get_children().is_empty(), true);
    }

    #[test]
    fn test_create_children_with_insertion_longer_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bonjour";

        let mut node = Node::new(FIRST_CHARACTERS);

        assert_eq!(node.get_children().is_empty(), true);

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        assert_eq!(node.get_children().len(), 2);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(node.get_characters(), "");
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);

        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_create_children_with_insertion_shorter_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bon";

        let mut node = Node::new(FIRST_CHARACTERS);

        assert_eq!(node.get_children().is_empty(), true);

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        assert_eq!(node.get_children().len(), 2);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(node.get_characters(), "");
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);

        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_get_children_from_root_are_empty_by_default() {

        let node = Node::new("hello");
        assert_eq!(node.get_children().is_empty(), true);
    }

    #[test]
    fn test_characters_exist_into_root_node() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().is_empty(), true);

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hella"), false);
        assert_eq!(node.exists("bonjour"), false);
    }

    #[test]
    fn test_characters_exist_into_root_node_with_small_word() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().is_empty(), true);

        assert_eq!(node.exists("he"), true);
        assert_eq!(node.exists("hey"), false);
    }

    #[test]
    fn test_characters_exist_into_root_node_with_long_word() {

        let node = Node::new("hello");

        assert_eq!(node.get_children().is_empty(), true);

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("helloworld"), false);
    }

    #[test]
    fn test_split_characters_at_the_beginning() {

        let mut node = Node::new("bonjour");
        node.insert("bien");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "b");
        assert_eq!(children[0].get_characters(), "onjour");
        assert_eq!(children[1].get_characters(), "ien");

        assert_eq!(children.len(), 2);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_split_characters_in_the_middle() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");

        assert_eq!(children.len(), 2);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_split_characters_at_the_end() {

        let mut node = Node::new("bona");
        node.insert("boni");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "a");
        assert_eq!(children[1].get_characters(), "i");

        assert_eq!(children.len(), 2);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_add_three_children_to_node() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");
        node.insert("bonsoir");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
        assert_eq!(children[2].get_characters(), "soir");

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

        assert_eq!(children.len(), 3);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
        assert_eq!(children[2].get_children().is_empty(), true);
    }

    #[test]
    fn test_add_four_children_to_node() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");
        node.insert("bonsoir");
        node.insert("bonnenuit");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
        assert_eq!(children[2].get_characters(), "soir");
        assert_eq!(children[3].get_characters(), "nenuit");

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

        assert_eq!(children.len(), 4);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
        assert_eq!(children[2].get_children().is_empty(), true);
        assert_eq!(children[3].get_children().is_empty(), true);
    }

    #[test]
    fn test_add_three_children_to_node_with_different_root() {

        let mut node = Node::new("jour");
        node.insert("app");
        node.insert("soir");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "");
        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
        assert_eq!(children[2].get_characters(), "soir");

        assert_eq!(node.exists("jour"), true);
        assert_eq!(node.exists("app"), true);
        assert_eq!(node.exists("soir"), true);

        assert_eq!(node.exists("journee"), false);
        assert_eq!(node.exists(" app"), false);
        assert_eq!(node.exists("siir"), false);

        assert_eq!(children.len(), 3);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
        assert_eq!(children[2].get_children().is_empty(), true);
    }

    #[test]
    fn test_add_four_children_to_node_with_different_root() {

        let mut node = Node::new("jour");
        node.insert("app");
        node.insert("soir");
        node.insert("neapp");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "");
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

        assert_eq!(children.len(), 4);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
        assert_eq!(children[2].get_children().is_empty(), true);
        assert_eq!(children[3].get_children().is_empty(), true);
    }

    #[test]
    fn test_update_first_child_characters() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lt");
            assert_eq!(children[1].get_characters(), "me");

            assert_eq!(node.get_children().len(), 2);
            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
        }

        node.insert("salted");

        {
            let children = node.get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lted");

            assert_eq!(node.get_children().len(), 2);
            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
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

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lt");
            assert_eq!(children[1].get_characters(), "me");

            assert_eq!(node.get_children().len(), 2);
            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
        }

        node.insert("sameless");

        {
            let children = node.get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lt");
            assert_eq!(children[1].get_characters(), "meless");

            assert_eq!(node.get_children().len(), 2);
            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
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

        assert_eq!(node.get_characters(), "hell");
        assert_eq!(children[0].get_characters(), "o");
        assert_eq!(children[1].get_characters(), "a");

        assert_eq!(node.get_children().len(), 2);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
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

        assert_eq!(node.get_characters(), "hello");
        assert_eq!(children[0].get_characters(), "world");
        assert_eq!(children[1].get_characters(), "earth");

        assert_eq!(node.get_children().len(), 2);
        assert_eq!(children[0].get_children().is_empty(), true);
        assert_eq!(children[1].get_children().is_empty(), true);
    }

    #[test]
    fn test_create_child_of_child() {

        let mut node = Node::new("salt");
        node.insert("same");

        {
            let children = node.get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lt");
            assert_eq!(children[1].get_characters(), "me");

            assert_eq!(children.len(), 2);

            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
        }

        node.insert("salted");

        {
            let children = node.get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lted");
            assert_eq!(children[1].get_characters(), "me");

            assert_eq!(children.len(), 2);

            assert_eq!(children[0].get_children().is_empty(), true);
            assert_eq!(children[1].get_children().is_empty(), true);
        }

        node.insert("saltandpepper");

        {
            let children = node.get_children();
            let sub_children = children[0].get_children();

            assert_eq!(node.get_characters(), "sa");
            assert_eq!(children[0].get_characters(), "lt");
            assert_eq!(sub_children[0].get_characters(), "ed");
            assert_eq!(sub_children[1].get_characters(), "andpepper");

            assert_eq!(children[1].get_children().is_empty(), true);
            assert_eq!(sub_children[0].get_children().is_empty(), true);
            assert_eq!(sub_children[1].get_children().is_empty(), true);
        }

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

        assert_eq!(node.get_characters(), "sa");
        assert_eq!(node.get_children().len(), 2);

        let children = node.get_children();

        assert_eq!(children[0].get_characters(), "lt");
        assert_eq!(children[0].get_children().len(), 3);

        assert_eq!(children[1].get_characters(), "me");
        assert_eq!(children[1].get_children().is_empty(), true);

        let first_child_subchildren = children[0].get_children();

        assert_eq!(first_child_subchildren[0].get_characters(), "ed");
        assert_eq!(first_child_subchildren[1].get_characters(), "andpepper");
        assert_eq!(first_child_subchildren[2].get_characters(), "o");

        assert_eq!(first_child_subchildren[0].get_children().is_empty(), true);
        assert_eq!(first_child_subchildren[1].get_children().is_empty(), true);
        assert_eq!(first_child_subchildren[2].get_children().is_empty(), true);
    }
}
