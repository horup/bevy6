pub enum GlobalCommand {
    /**Clear the world, removing all entities which  */
    ClearWorld,
    /**Save world to path */
    SaveWorld {
        path:String
    },
    /**Load world from path */
    LoadWorld {
        path:String
    }
}