#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserData {
	None = 255,

	Nil = 0,
	Bool,
	LightUserData,
	Number,
	String,
	Table,
	Function,
	UserData,
	Thread,

	// GMod Types
	Entity,
	Vector,
	Angle,
	PhysObj,
	Save,
	Restore,
	DamageInfo,
	EffectData,
	MoveData,
	RecipientFilter,
	UserCmd,
	ScriptedVehicle,
	Material,
	Panel,
	Particle,
	ParticleEmitter,
	Texture,
	UserMsg,
	ConVar,
	IMesh,
	Matrix,
	Sound,
	PixelVisHandle,
	DLight,
	Video,
	File,
	Locomotion,
	Path,
	NavArea,
	SoundHandle,
	NavLadder,
	ParticleSystem,
	ProjectedTexture,
	PhysCollide,
	SurfaceInfo,

	MAX
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector {
	x: f32,
	y: f32,
	z: f32
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Angle {
	p: f32,
	y: f32,
	r: f32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TaggedUserData
{
	pub data: *mut core::ffi::c_void,
	pub r#type: UserData
}

pub trait CoercibleUserData {}

macro_rules! userdata {
	($(UserData::$enum:ident => $struct:ident),+) => {
		$(impl CoercibleUserData for $struct {})+

		impl TaggedUserData {
			/// Coerce this tagged UserData into its corresponding Rust struct, if possible.
			///
			/// This will perform a type check to ensure that the tagged userdata matches the user data you are coercing to.
			pub fn coerce<'a, T: CoercibleUserData>(&self) -> Result<&mut T, UserData> {
				match self.r#type {
					$(UserData::$enum => Ok(unsafe { &mut *(self.data as *mut T) }),)+
					_ => Err(self.r#type)
				}
			}

			/// Coerce this tagged UserData into its corresponding Rust struct, if possible.
			///
			/// # Safety
			/// This will NOT perform a type check to ensure that the tagged userdata matches the user data you are coercing to.
			///
			/// Coercing to the wrong type is undefined behaviour and is likely to crash your program.
			pub unsafe fn coerce_unchecked<T: CoercibleUserData>(&self) -> &mut T {
				&mut *(self.data as *mut T)
			}
		}
	};
}
userdata! {
	UserData::Vector => Vector,
	UserData::Angle => Angle
}