use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigData {
    ConfigExe: ConfigExe,
    ConfigAir: ConfigAir,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigExe {
    Image1: ImageInfo,
    Image2: ImageInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageInfo {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigAir {
    SocketServer: SocketServer,
    WebSocketServer: WebSocketServer,
    ImgSplit: ImgSplit,
    Type: Type,
    Template: Template,
}

#[derive(Serialize, Deserialize, Debug)]
struct SocketServer {
    IP: String,
    Port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct WebSocketServer {
    Port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImgSplit {
    CountX: u16,
    CountY: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Type {
    Path: String,
    Data: Vec<TypeData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TypeData {
    name: String,
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    PicNum: TemplatePicNum,
    PicEn: TemplatePicEn,
    PicPk: TemplatePicPk,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicNum {
    Path: String,
    Data: Vec<TemplatePicNumData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicNumData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicEn {
    Path: String,
    Data: Vec<TemplatePicEnData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicEnData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicPk {
    Path: String,
    Data: Vec<TemplatePicPkData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePicPkData {
    name: String,
    label: String,
    color: String,
}

fn pt<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
