use std::error;
use std::io;
use std::rc;

use crate::binary_reader;
use crate::soundfont_info::SoundFontInfo;
use crate::soundfont_sampledata::SoundFontSampleData;
use crate::soundfont_parameters::SoundFontParameters;

#[non_exhaustive]
pub struct SoundFont
{
    pub info: SoundFontInfo,
    pub bits_per_sample: i32,
    pub wave_data: rc::Rc<Vec<i16>>,
}

impl SoundFont
{
    pub fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let chunk_id = match binary_reader::read_four_cc(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if chunk_id != "RIFF"
        {
            return Err(format!("The RIFF chunk was not found.").into());
        }

        let _size = binary_reader::read_i32(reader);

        let form_type = match binary_reader::read_four_cc(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if form_type != "sfbk"
        {
            return Err(format!("The type of the RIFF chunk must be 'sfbk', but was '{form_type}'.").into());
        }

        let info = match SoundFontInfo::new(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let sample_data = match SoundFontSampleData::new(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let result = SoundFontParameters::new(reader);

        Ok(SoundFont
        {
            info: info,
            bits_per_sample: 16,
            wave_data: rc::Rc::new(sample_data.wave_data),
        })
    }
}