use rand::rngs::SmallRng;
use rand::*;
use rand::{Rng, SeedableRng};
#[derive(Debug)]
enum TypeNode {
    // 0
    NonZeroU8,
    // 1
    NonZeroI8,
    // 2
    NonZeroU16,
    // 3
    NonZeroI16,
    // 4
    NonZeroU32,
    // 5
    NonZeroI32,
    // 6
    NonZeroU64,
    // 7
    NonZeroI64,
    // 8
    NonZeroU128,
    // 9
    NonZeroI128,
    // 10
    NonZeroUSize,
    // 11
    NonZeroISize,
    // 12 - 20
    NonNull(Box<Self>),
    // 20..30
    Tuple(Box<[Self]>),
    // 31
    U8,
    // 32
    I8,
    // 33
    U16,
    // 34
    I16,
    // 35
    U32,
    // 36
    I32,
    // 37
    U64,
    // 38
    I64,
    // 39
    U128,
    // 40
    I128,
    // 41
    USize,
    // 42
    ISize,
    // 43 
    F32,
    // 44
    F64,
    //45..50
    Array{element:Box<Self>,length:u32},
    // 50..60
    Option(Box<Self>),
    // 50..60
    Result(Box<Self>,Box<Self>),
}
impl TypeNode {
    pub fn random_tychain_<R: rand::Rng>(rng: &mut R, overflow_guard: u16) -> Self {
        let val = rng.gen_range(0..=70);
        match val {
            0 => Self::NonZeroU8,
            1 => Self::NonZeroI8,
            2 => Self::NonZeroU16,
            3 => Self::NonZeroI16,
            4 => Self::NonZeroU32,
            5 => Self::NonZeroI32,
            6 => Self::NonZeroU64,
            7 => Self::NonZeroI64,
            8 => Self::NonZeroU128,
            9 => Self::NonZeroI128,
            10 => Self::NonZeroUSize,
            11 => Self::NonZeroISize,
            12..=19 => Self::NonNull(Self::random_tychain_::<R>(rng, overflow_guard + 1).into()),
            20..=30 => {
                let length = if overflow_guard < 6 {
                    (rng.gen_range(0..=256) as f64).log2() as i32
                } else {
                    0
                };
                Self::Tuple(
                    (0..length)
                        .into_iter()
                        .map(|_| Self::random_tychain_::<R>(rng, overflow_guard + 1))
                        .collect(),
                )
            }
            31 => Self::U8,
            32 => Self::I8,
            33 => Self::U16,
            34 => Self::I16,
            35 => Self::U32,
            36 => Self::I32,
            37 => Self::U64,
            38 => Self::I64,
            39 => Self::U128,
            40 => Self::I128,
            41 => Self::USize,
            42 => Self::ISize,
            43 => Self::F32,
            44 => Self::F64,
            45..=50 => {
                let element = Self::random_tychain_::<R>(rng, overflow_guard + 1).into();
                Self::Array{
                    element:Box::new(element),
                    length:((rng.gen_range(0..=1024) as f64).log2() as u32),
                }
            }
            51..=60=>Self::Option(Self::random_tychain_::<R>(rng, overflow_guard + 1).into()),
            61..=70=>Self::Result(Self::random_tychain_::<R>(rng, overflow_guard + 1).into(),Self::random_tychain_::<R>(rng, overflow_guard + 1).into()),
            _ => todo!("Unhandled val:{val}"),
        }
    }
    pub fn random_tychain(mut rng: impl rand::Rng) -> Self {
        Self::random_tychain_(&mut rng, 0)
    }
    pub fn type_string(&self)->String{
        match self{
            TypeNode::NonZeroU8 => "NonZeroU8".into(),
            TypeNode::NonZeroI8 => "NonZeroI8".into(),
            TypeNode::NonZeroU16 => "NonZeroU16".into(),
            TypeNode::NonZeroI16 => "NonZeroI16".into(),
            TypeNode::NonZeroU32 => "NonZeroU32".into(),
            TypeNode::NonZeroI32 => "NonZeroI32".into(),
            TypeNode::NonZeroU64 => "NonZeroU64".into(),
            TypeNode::NonZeroI64 => "NonZeroI64".into(),
            TypeNode::NonZeroU128 => "NonZeroU128".into(),
            TypeNode::NonZeroI128 => "NonZeroI128".into(),
            TypeNode::NonZeroUSize => "NonZeroUsize".into(),
            TypeNode::NonZeroISize => "NonZeroIsize".into(),
            TypeNode::NonNull(inner) => format!("NonNull<{inner}>",inner = inner.type_string()),
            TypeNode::Tuple(elements) => {
                let mut res:String = "(".into();
                for element in elements.iter(){
                    res.push_str(&element.type_string());
                    res.push(',');
                }
                res.push(')');
                res
            }
            TypeNode::U8 => "u8".to_string(),
            TypeNode::I8 => "i8".to_string(),
            TypeNode::U16 => "u16".to_string(),
            TypeNode::I16 =>  "i16".to_string(),
            TypeNode::U32 =>  "u32".to_string(),
            TypeNode::I32 =>  "i32".to_string(),
            TypeNode::U64 =>  "u64".to_string(),
            TypeNode::I64 =>  "i64".to_string(),
            TypeNode::U128 => "u128".to_string(),
            TypeNode::I128 => "i128".to_string(),
            TypeNode::USize => "usize".to_string(),
            TypeNode::ISize => "isize".to_string(),
            TypeNode::F32 => "f32".to_string(),
            TypeNode::F64 => "f64".to_string(),
            TypeNode::Array { element, length } =>  format!("[{element};{length}]",element = element.type_string()),
            TypeNode::Option(inner) => format!("Option<{inner}>",inner = inner.type_string()),
            TypeNode::Result(err, ok) =>  format!("Result<{ok},{err}>",err = err.type_string(),ok = ok.type_string()),
        }
    }
    pub fn random_value<R: rand::Rng>(&self,rng: &mut R)->String{
        match self{
            TypeNode::NonZeroU8 => format!("NonZeroU8::new({value}).unwrap()",value = rng.gen_range(1..u8::MAX)),
            TypeNode::NonZeroI8 => {
                let value = rng.gen::<i8>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroI8::new({value}).unwrap()")
            },
            TypeNode::NonZeroU16 => format!("NonZeroU16::new({value}).unwrap()",value = rng.gen_range(1..u16::MAX)),
            TypeNode::NonZeroI16 => {
                let value = rng.gen::<i16>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroI16::new({value}).unwrap()")
            },
            TypeNode::NonZeroU32 => format!("NonZeroU32::new({value}).unwrap()",value = rng.gen_range(1..u32::MAX)),
            TypeNode::NonZeroI32 =>{
                let value = rng.gen::<i32>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroI32::new({value}).unwrap()")
            },
            TypeNode::NonZeroU64 => format!("NonZeroU64::new({value}).unwrap()",value = rng.gen_range(1..u64::MAX)),
            TypeNode::NonZeroI64 => {
                let value = rng.gen::<i64>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroI64::new({value}).unwrap()")
            },
            TypeNode::NonZeroU128 => format!("NonZeroU128::new({value}).unwrap()",value = rng.gen_range(1..u128::MAX)),
            TypeNode::NonZeroI128 =>  {
                let value = rng.gen::<i128>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroI128::new({value}).unwrap()")
            },
            TypeNode::NonZeroUSize => format!("NonZeroUsize::new({value}).unwrap()",value = rng.gen_range(1..u64::MAX)),
            TypeNode::NonZeroISize =>  {
                let value = rng.gen::<i64>();
                let value = if value == 0{
                    1
                }else{value};
                format!("NonZeroIsize::new({value}).unwrap()")
            },
            TypeNode::NonNull(inner) => format!("NonNull::new({value} as *mut {type_string}).unwrap()",value = rng.gen_range(1..u64::MAX),type_string = inner.type_string()),
            TypeNode::Tuple(elements) => {
                let mut res:String = "(".into();
                for element in elements.iter(){
                    res.push_str(&element.random_value(rng));
                    res.push(',');
                }
                res.push(')');
                res
            }
            TypeNode::U8 => format!("{value}",value = rng.gen::<u8>()),
            TypeNode::I8 => format!("{value}",value = rng.gen::<i8>()),
            TypeNode::U16 => format!("{value}",value = rng.gen::<u16>()),
            TypeNode::I16 => format!("{value}",value = rng.gen::<i16>()),
            TypeNode::U32 => format!("{value}",value = rng.gen::<u32>()),
            TypeNode::I32 => format!("{value}_i32",value = rng.gen::<i32>()),
            TypeNode::U64 => format!("{value}",value = rng.gen::<u64>()),
            TypeNode::I64 => format!("{value}",value = rng.gen::<i64>()),
            TypeNode::U128 => format!("{value}",value = rng.gen::<u128>()),
            TypeNode::I128 => format!("{value}",value = rng.gen::<i128>()),
            TypeNode::USize => format!("{value}",value = rng.gen::<u64>()),
            TypeNode::ISize => format!("{value}",value = rng.gen::<i64>()),
            TypeNode::F32 => format!("{value}",value = rng.gen::<f32>()),
            TypeNode::F64 => format!("{value}",value = rng.gen::<f64>()),
            TypeNode::Array { element, length } => {
                let mut res:String = "[".into();
                for _ in 0..(*length){
                    res.push_str(&element.random_value(rng));
                    res.push(',');
                }
                res.push(']');
                res
            }
            TypeNode::Option(inner) => match rng.gen_bool(0.8){
                true => format!("Some({inner})",inner = inner.random_value(rng)),
                false => "None".into(),
            }
            TypeNode::Result(err, ok) => match rng.gen_bool(0.5){
                true => format!("Err::<{ok},{err}>({err_val})",err_val = err.random_value(rng),err = err.type_string(),ok = ok.type_string()),
                false => format!("Ok::<{ok},{err}>({ok_val})",ok_val = ok.random_value(rng),err = err.type_string(),ok = ok.type_string()),
            }
        }
    }
}
const TESTS_PER_TYPE:u32 = 10;
fn test_type(tychain:&TypeNode,id:u32,string:&mut String){
    string.push_str(&format!("fn test{id}(){{"));
    string.push_str(&format!("let val:{val_ty} = {val};\n",val = tychain.random_value(&mut SmallRng::from_entropy()),val_ty = tychain.type_string()));
    string.push_str(&format!("let size = std::mem::size_of_val(&val);\n"));
    string.push_str(&format!("let align = std::mem::align_of_val(&val);\n"));
    string.push_str(&format!("unsafe{{printf(\"\\ntype:{ty_id};test:{test_id};size:%x;align:%x;raw_bytes:\\0\".as_ptr() as *const i8,size,align)}};\n",ty_id = id / TESTS_PER_TYPE,test_id = id%TESTS_PER_TYPE));
    string.push_str(&format!("let mut idx = 0;\n"));
    string.push_str(&format!("while(idx < size){{\n\tunsafe{{printf(\"%x\\0\".as_ptr() as *const i8,*((&val as *const _ as *const u8).add(idx)) as u32)}}; \n"));
    string.push_str(&format!("idx += 1;\n}}\nunsafe{{printf(\";\\0\".as_ptr() as *const i8);}}\n"));
    string.push_str("}");
}
fn main() {
    let tychain = TypeNode::random_tychain(SmallRng::from_entropy());
    let mut res = "extern crate core;
    use core::num::{NonZeroI16,NonZeroI128,NonZeroU8,NonZeroI32,NonZeroI8,NonZeroU128,NonZeroU32,NonZeroI64,NonZeroU16,NonZeroU64,NonZeroUsize,NonZeroIsize};
    use core::ptr::NonNull;
    use core::ffi::{c_char, c_int};
    extern \"C\" {
        fn printf(fmt: *const c_char, ...) -> c_int;
    }".to_string();
    let mut main_str:String = "fn main(){".into();
    let type_id = 0;
    for id in 0..TESTS_PER_TYPE{
        test_type(&tychain,id + type_id*TESTS_PER_TYPE,&mut res);
        main_str.push_str(&format!("test{}();\n",id + type_id*TESTS_PER_TYPE));
    }
    main_str.push('}');
    println!("{res}{main_str}");
    //println!("let val:{type_string} = {val}",val = tychain.random_value(&mut SmallRng::from_entropy()),type_string = tychain.type_string());
    //println!("tychain:{tychain:?}",tychain = TypeNode::random_tychain(SmallRng::from_entropy()).random_value(&mut SmallRng::from_entropy()));
    //println!("tychain:{tychain:?}",tychain = TypeNode::random_tychain(SmallRng::from_entropy()).random_value(&mut SmallRng::from_entropy()));
    //println!("tychain:{tychain:?}",tychain = TypeNode::random_tychain(SmallRng::from_entropy()).random_value(&mut SmallRng::from_entropy()));
}
