use oxidizemc::build::ModLoaderConfig;

pub const FABRICMC_MOD_LOADER: ModLoaderConfig = ModLoaderConfig {
    name: "FabricMC",
    lib_name: "oxidizemc-fabric",
    mod_initializers: &["ModInitializer", "ClientModInitializer", "DedicatedServerModInitializer"],
};
