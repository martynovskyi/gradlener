
#[derive(Clone, Debug)]
pub struct Project {
    pub plugins: Plugins,
    pub dependencies: Dependencies,
}

#[derive(Clone, Debug)]
pub struct Plugins {
    pub entries: [Plugin; 0],
}

#[derive(Clone, Debug)]
pub struct Plugin {
    pub id: String,
    pub version: String,
    pub apply: bool,
}

#[derive(Clone, Debug)]
pub struct Dependencies {
    pub entires: [Dependency; 0],
}


#[derive(Clone, Debug)]
pub struct Dependency {
    pub group: String,
    pub artifact: String,
    pub version: String,
}
