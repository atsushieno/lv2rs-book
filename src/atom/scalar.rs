use crate::atom::AtomBody;
use crate::ports::AtomOutputPort;
use crate::uris;
use crate::writer::RawWriter;
use std::ffi::CStr;

pub trait ScalarAtomBody {
    fn construct_body<'a>(writer: &mut RawWriter<'a>, value: &Self) -> Result<&'a mut Self, ()>
    where
        Self: Sized,
    {
        Ok(writer.write_sized(value, true)?.0)
    }
}

impl<A: AtomBody + Clone + ?Sized> AtomOutputPort<A>
where
    A: ScalarAtomBody,
{
    pub fn write_atom<'a>(
        &'a mut self,
        value: &A,
        urid: &uris::MappedURIDs,
    ) -> Result<&'a mut A, ()> {
        let mut writer = self.get_writer()?;
        writer.push_atom_header::<A>(urid)?;
        A::construct_body(&mut writer, value)
    }
}

pub use std::os::raw::c_int;

impl AtomBody for c_int {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.int
    }
}

impl ScalarAtomBody for c_int {}

pub use std::os::raw::c_long;

impl AtomBody for c_long {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.long
    }
}

impl ScalarAtomBody for c_long {}

pub use std::os::raw::c_float;

impl AtomBody for c_float {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.float
    }
}

impl ScalarAtomBody for c_float {}

pub use std::os::raw::c_double;

impl AtomBody for c_double {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.double
    }
}

impl ScalarAtomBody for c_double {}

pub use urid::URID;

impl AtomBody for URID {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.urid
    }
}

impl ScalarAtomBody for URID {}

impl AtomBody for bool {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::BOOL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.bool
    }
}

impl ScalarAtomBody for bool {}
