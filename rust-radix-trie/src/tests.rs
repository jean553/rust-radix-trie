#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation_and_insertion() {

        let mut node = Node::new("hello");

        const INSERTED_CHARACTERS: &str = "helloworld";
        node.insert(INSERTED_CHARACTERS);
        assert_eq!(node.get_characters(), INSERTED_CHARACTERS);
    }

    #[test]
    fn test_create_children_with_insertion_longer_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bonjour";

        let mut node = Node::new(FIRST_CHARACTERS);

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);
        assert_eq!(node.get_characters(), "");
    }

    #[test]
    fn test_create_children_with_insertion_shorter_than_characters() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bon";

        let mut node = Node::new(FIRST_CHARACTERS);

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);
        assert_eq!(node.get_characters(), "");
    }

    #[test]
    fn test_get_children_from_root_are_empty_by_default() {

        let node = Node::new("hello");
        assert_eq!(node.get_children().is_empty(), true);
    }

    #[test]
    fn test_characters_exist_into_root_node() {

        let node = Node::new("hello");

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("bonjour"), false);
    }

    #[test]
    fn test_split_characters_at_the_beginning() {

        let mut node = Node::new("bonjour");
        node.insert("bien");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "b");
        assert_eq!(children[0].get_characters(), "onjour");
        assert_eq!(children[1].get_characters(), "ien");
    }

    #[test]
    fn test_split_characters_in_the_middle() {

        let mut node = Node::new("bonjour");
        node.insert("bonapp");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "jour");
        assert_eq!(children[1].get_characters(), "app");
    }

    #[test]
    fn test_split_characters_at_the_end() {

        let mut node = Node::new("bona");
        node.insert("boni");

        let children = node.get_children();

        assert_eq!(node.get_characters(), "bon");
        assert_eq!(children[0].get_characters(), "a");
        assert_eq!(children[1].get_characters(), "i");
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
    }

    #[test]
    fn test_characters_exist_into_root_node_when_children_exist() {

        let mut node = Node::new("hello");
        node.insert("hella");

        assert_eq!(node.exists("hell"), true);
    }

    #[test]
    fn test_characters_exist_into_children_node_when_children_exist() {

        let mut node = Node::new("hello");
        node.insert("hella");

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("hella"), true);
    }
}
