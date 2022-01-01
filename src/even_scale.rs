use utau_rs::*;

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Scale{
    C,C_sharp,Cm,Cm_sharp,
    D,D_sharp,Dm,Dm_sharp,
    E,E_sharp,Em,Em_sharp,
    F,F_sharp,Fm,Fm_sharp,
    G,G_sharp,Gm,Gm_sharp,
    A,A_sharp,Am,Am_sharp,
    B,B_sharp,Bm,Bm_sharp,
}

impl Scale{
    pub const ALL: [Scale;28]=[
        Scale::C,Scale::C_sharp,Scale::Cm,Scale::Cm_sharp,
        Scale::D,Scale::D_sharp,Scale::Dm,Scale::Dm_sharp,
        Scale::E,Scale::E_sharp,Scale::Em,Scale::Em_sharp,
        Scale::F,Scale::F_sharp,Scale::Fm,Scale::Fm_sharp,
        Scale::G,Scale::G_sharp,Scale::Gm,Scale::Gm_sharp,
        Scale::A,Scale::A_sharp,Scale::Am,Scale::Am_sharp,
        Scale::B,Scale::B_sharp,Scale::Bm,Scale::Bm_sharp,
    ];
}

impl Default for Scale{
    fn default()->Self{
        Scale::C
    }
}

impl std::fmt::Display for Scale{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{}",match self{
            Scale::C=>"C",Scale::C_sharp=>"C#",Scale::Cm=>"Cm",Scale::Cm_sharp=>"C#m",
            Scale::D=>"D",Scale::D_sharp=>"D#",Scale::Dm=>"Dm",Scale::Dm_sharp=>"D#m",
            Scale::E=>"E",Scale::E_sharp=>"E#",Scale::Em=>"Em",Scale::Em_sharp=>"E#m",
            Scale::F=>"F",Scale::F_sharp=>"F#",Scale::Fm=>"Fm",Scale::Fm_sharp=>"F#m",
            Scale::G=>"G",Scale::G_sharp=>"G#",Scale::Gm=>"Gm",Scale::Gm_sharp=>"G#m",
            Scale::A=>"A",Scale::A_sharp=>"A#",Scale::Am=>"Am",Scale::Am_sharp=>"A#m",
            Scale::B=>"B",Scale::B_sharp=>"B#",Scale::Bm=>"Bm",Scale::Bm_sharp=>"B#m",
        })
    }
}

pub struct KeyTone([u32;7]);

const C: u32=24;
const D: u32=26;
const E: u32=28;
const F: u32=29;
const G: u32=31;
const A: u32=33;
const B: u32=35;

impl KeyTone{
    pub fn new(scale: &Scale)->Result<KeyTone,&'static str>{
        Ok(match scale{
            Scale::C      |Scale::Am      =>KeyTone([C,D,E,F,G,A,B]),
            Scale::C_sharp|Scale::Am_sharp=>KeyTone([C+1,D+1,E+1,F+1,G+1,A+1,B+1]),
            Scale::D      |Scale::Bm      =>KeyTone([C+1,D,E,F+1,G,A,B]),
            Scale::D_sharp|Scale::Cm      =>KeyTone([C,D,E-1,F,G,A-1,B-1]),
            Scale::E      |Scale::Cm_sharp=>KeyTone([C+1,D+1,E,F+1,G+1,A,B]),
            Scale::F      |Scale::Dm      =>KeyTone([C,D,E,F,G,A,B-1]),
            Scale::F_sharp|Scale::Dm_sharp=>KeyTone([C+1,D+1,E+1,F+1,G+1,A+1,B]),
            Scale::G      |Scale::Em      =>KeyTone([C,D,E,F+1,G,A,B]),
            Scale::G_sharp|Scale::Fm      =>KeyTone([C,D-1,E-1,F,G,A-1,B-1]),
            Scale::A      |Scale::Fm_sharp=>KeyTone([C+1,D,E,F+1,G+1,A,B]),
            Scale::A_sharp|Scale::Gm      =>KeyTone([C,D,E-1,F,G,A,B-1]),
            Scale::B      |Scale::Gm_sharp=>KeyTone([C+1,D+1,E,F+1,G+1,A+1,B]),
            _=>return Err("不明なエラーが発生しました."),
        })
    }

    fn unwrap(&self)->[u32;7]{
        match self{
            &KeyTone(some)=>some,
        }
    }
}

pub fn even_scale(uta_sections: &mut UtaSections,scale: &Scale)->Result<(),&'static str>{
    let tones=KeyTone::new(scale).unwrap();

    for section in uta_sections.sections.iter_mut(){
        if tones.unwrap().iter().all(|&x|(x%section.note_num)!=0){
            let mut near=section.note_num as i32-tones.unwrap()[0] as i32;
            for tone in tones.unwrap(){
                let tone=match section.note_num{
                    24..=35=>tone+12*0,
                    36..=47=>tone+12*1,
                    48..=59=>tone+12*2,
                    60..=71=>tone+12*3,
                    72..=83=>tone+12*4,
                    84..=95=>tone+12*5,
                    96..=107=>tone+12*6,
                    _=>return Err("不明なエラーが発生しました."),
                };
                if (section.note_num as i32-tone as i32).abs()<near{
                    near=(section.note_num as i32-tone as i32).abs();
                }
            }
            section.note_num=(section.note_num as i32+near) as u32;
        }
    };

    Ok(())
}