//! The LEGO data format
#[cfg(feature = "serde-derives")]
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

/// A LEGO-Data-Format value
#[derive(PartialEq)]
pub enum Value {
    /// A user-facing string
    String(String),
    /// A signed 32bit integer
    I32(i32),
    /// A single precision floating point number
    F32(f32),
    /// An unsigned 32bit interger
    U32(u32),
    /// A boolean (0 or 1)
    Bool(bool),
    /// An internal string
    Bytes(String),
}

#[cfg(feature = "serde-derives")]
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s.as_str()),
            Self::I32(i) => serializer.serialize_i32(*i),
            Self::F32(i) => serializer.serialize_f32(*i),
            Self::U32(i) => serializer.serialize_u32(*i),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Bytes(b) => serializer.serialize_str(b.as_str()),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::I32(i) => i.fmt(f),
            Self::F32(l) => l.fmt(f),
            Self::U32(u) => u.fmt(f),
            Self::Bool(b) => b.fmt(f),
            Self::Bytes(s) => {
                write!(f, "b")?;
                s.fmt(f)
            }
        }
    }
}

/// A table of LDF values
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[cfg_attr(feature = "serde-derives", serde(transparent))]
pub struct LDF {
    /// The contained map
    pub map: BTreeMap<String, Value>,
}

impl Debug for LDF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("LDF");
        for (name, value) in self.map.iter() {
            d.field(name, value);
        }
        d.finish()
    }
}

/// Error when parsing LDF
#[derive(Debug)]
pub struct LDFError(u8, String);

impl FromStr for LDF {
    type Err = LDFError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('\n')
            .try_fold(BTreeMap::new(), |mut x, y| {
                let mut out = y.splitn(2, '=');
                let key = out.next().unwrap();
                let val = out.next().ok_or_else(|| LDFError(0, key.to_string()))?;
                let mut inn = val.splitn(2, ':');
                let typ = inn.next().unwrap();
                let z = inn.next().ok_or_else(|| LDFError(1, key.to_string()))?;

                let v = match typ {
                    "0" => Value::String(z.into()),
                    "1" => Value::I32(z.parse().map_err(|_| LDFError(2, key.to_string()))?),
                    "3" => Value::F32(z.parse().map_err(|_| LDFError(3, z.to_string()))?),
                    "5" => Value::U32(z.parse().map_err(|_| LDFError(4, key.to_string()))?),
                    "7" => Value::Bool(match z {
                        "0" => false,
                        "1" => true,
                        _ => return Err(LDFError(5, key.to_string())),
                    }),
                    "13" => Value::Bytes(z.into()),
                    _ => return Err(LDFError(6, key.to_string())),
                };
                x.insert(key.to_string(), v);
                Ok(x)
            })
            .map(|map| LDF { map })
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, LDF};

    #[test]
    fn test_from_str() {
        let text = "CamRlBiasAmt=3:0\nCamRlBiasBi=7:0\nCamRlBiasFwd=7:0\nCamRlBiasRet=7:0\nCamRlCamPath=0:\nCamRlCamRetPath=0:\nCamRlFaceTgt=7:1\nCamRlPosPath=0:\nCheckPrecondition=0:\nNDAEG=13:\nadd_to_navmesh=7:1\ncamBehaviorDirectional=7:0\ncamBehaviorPitch=7:0\ncamBehaviorYaw=7:0\ncamDir=0:\ncamGradSnap=7:0\ncamLkDir=7:0\ncamPitchAngleDown=3:120\ncamPitchAngleUp=3:60\ncamPrefersToFadeObject=7:1\ncancelBehaviorMovement=7:0\ncarver_only=7:0\ncreate_physics=7:1\ncustom_config_names=0:\nfxpriority=1:0\nignoreCameraCollision=7:0\ninteraction_distance=3:16\nis_smashable=7:0\nloadOnClientOnly=7:0\nloadSrvrOnly=7:0\nnavmesh_carver=7:0\nrenderCullingGroup=5:0\nrespawnVol=7:0\nrespawnVolName=0:\nrlLeadIn=3:1\nrlLeadOut=3:-1\nrspPos=0:0.0000\u{1f}0.0000\u{1f}0.0000\nrspRot=0:1.0000 \u{1f} 0.0000 \u{1f} 0.0000 \u{1f} 0.0000\nsceneIDOverride=1:0\nsceneIDOverrideEnabled=7:1\nsceneLayerIDOverride=5:0\nsetsRailCam=7:0\ntemplate=1:-1";
        let map: LDF = text.parse().unwrap();

        let mut res = vec![
            ("CamRlBiasAmt", Value::F32(0.0)),
            ("CamRlBiasBi", Value::Bool(false)),
            ("CamRlBiasFwd", Value::Bool(false)),
            ("CamRlBiasRet", Value::Bool(false)),
            ("CamRlCamPath", Value::String(String::new())),
            ("CamRlCamRetPath", Value::String(String::new())),
            ("CamRlFaceTgt", Value::Bool(true)),
            ("CamRlPosPath", Value::String(String::new())),
            ("CheckPrecondition", Value::String(String::new())),
            ("NDAEG", Value::Bytes(String::new())),
            ("add_to_navmesh", Value::Bool(true)),
            ("camBehaviorDirectional", Value::Bool(false)),
            ("camBehaviorPitch", Value::Bool(false)),
            ("camBehaviorYaw", Value::Bool(false)),
            ("camDir", Value::String(String::new())),
            ("camGradSnap", Value::Bool(false)),
            ("camLkDir", Value::Bool(false)),
            ("camPitchAngleDown", Value::F32(120.0)),
            ("camPitchAngleUp", Value::F32(60.0)),
            ("camPrefersToFadeObject", Value::Bool(true)),
            ("cancelBehaviorMovement", Value::Bool(false)),
            ("carver_only", Value::Bool(false)),
            ("create_physics", Value::Bool(true)),
            ("custom_config_names", Value::String(String::new())),
            ("fxpriority", Value::I32(0)),
            ("ignoreCameraCollision", Value::Bool(false)),
            ("interaction_distance", Value::F32(16.0)),
            ("is_smashable", Value::Bool(false)),
            ("loadOnClientOnly", Value::Bool(false)),
            ("loadSrvrOnly", Value::Bool(false)),
            ("navmesh_carver", Value::Bool(false)),
            ("renderCullingGroup", Value::U32(0)),
            ("respawnVol", Value::Bool(false)),
            ("respawnVolName", Value::String(String::new())),
            ("rlLeadIn", Value::F32(1.0)),
            ("rlLeadOut", Value::F32(-1.0)),
            (
                "rspPos",
                Value::String("0.0000\u{1f}0.0000\u{1f}0.0000".to_string()),
            ),
            (
                "rspRot",
                Value::String("1.0000 \u{1f} 0.0000 \u{1f} 0.0000 \u{1f} 0.0000".to_string()),
            ),
            ("sceneIDOverride", Value::I32(0)),
            ("sceneIDOverrideEnabled", Value::Bool(true)),
            ("sceneLayerIDOverride", Value::U32(0)),
            ("setsRailCam", Value::Bool(false)),
            ("template", Value::I32(-1)),
        ];

        let mut r = res.drain(..);
        for (key, value) in &map.map {
            let (rk, rv) = r.next().unwrap();
            assert_eq!((rk, &rv), (key.as_str(), value));
        }

        assert_eq!(r.len(), 0);
    }
}
