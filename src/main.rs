pub struct NewComputer
{
    pub memory:             Vec<usize>,
    pub program_position:   usize,

    pub buffer:             [usize; 0x4],
    pub colormap_table:     Vec<usize>,

    pub display_size:       usize,
    pub vram_offset:        usize,
    pub display_scale_by:   usize,

    pub input_offset:       usize,
    pub character_table_offset:     usize,
}

impl NewComputer
{
    pub fn process_instruction(&mut self)
    {
        let read = self.memory[self.program_position as usize];
        let instruction: u8 = ((read & 0xf0000000) >> 28) as u8;
        let memory_address = (read & 0x03ffffff) as usize;

        let buffer_address_a = (read & 0x0c000000 >> 26) as u8;
        let buffer_address_b = (read & 0x03000000 >> 24) as u8;
        let buffer_address_c = (read & 0x00c00000 >> 22) as u8;
        match instruction
        {
            0x0 => self.program_position = self.program_position.wrapping_add(1),
            0x1 =>
            {
                self.buffer[3] = self.program_position;
                self.program_position = memory_address;
            },
            0x2 =>
            {
                self.buffer[3] = self.program_position;
                if self.buffer[buffer_address_a as usize] == 0xffffffff
                {
                    self.program_position = memory_address;
                }
            },
            0x3 =>
            {
                self.buffer[3] = self.program_position;
                self.program_position = self.buffer[buffer_address_a as usize];
            }
            0x4 =>
            {
                self.buffer[buffer_address_a as usize] = self.memory[memory_address as usize];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0x5 =>
            {
                self.memory[memory_address as usize] = self.memory[self.buffer[buffer_address_a as usize] & 0x1ffffff];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0x6 =>
            {
                match self.buffer[buffer_address_a as usize] > self.buffer[buffer_address_b as usize]
                {
                    true =>     self.buffer[buffer_address_c as usize] = 0xffffffff,
                    false =>    self.buffer[buffer_address_c as usize] = 0x00000000,
                }
                self.program_position = self.program_position.wrapping_add(1)
            },
            0x7 =>
            {
                match self.buffer[buffer_address_a as usize] < self.buffer[buffer_address_b as usize]
                {
                    true =>     self.buffer[buffer_address_c as usize] = 0xffffffff,
                    false =>    self.buffer[buffer_address_c as usize] = 0x00000000,
                }
                self.program_position = self.program_position.wrapping_add(1)
            },
            0x8 =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize] & self.buffer[buffer_address_b as usize];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0x9 =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize] | self.buffer[buffer_address_b as usize];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xa =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize] ^ self.buffer[buffer_address_b as usize];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xb =>
            {
                self.buffer[buffer_address_c as usize] = !self.buffer[buffer_address_a as usize] | !self.buffer[buffer_address_b as usize];
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xc =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize].wrapping_add(self.buffer[buffer_address_b as usize]);
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xd =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize].wrapping_sub(self.buffer[buffer_address_b as usize]);
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xe =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize].unbounded_shl(u32::from_usize(self.buffer[buffer_address_b as usize])[0]);
                self.program_position = self.program_position.wrapping_add(1);
            },
            0xf =>
            {
                self.buffer[buffer_address_c as usize] = self.buffer[buffer_address_a as usize].unbounded_shr(u32::from_usize(self.buffer[buffer_address_b as usize])[0]);
                self.program_position = self.program_position.wrapping_add(1);
            },
            _   => ()
        }
    }

    pub fn calculate_color_corrections(&mut self)
    {
        let color_palette: [i32; 32] = [0x3a2658, 0x403287, 0x563341, 0x4d43bd, 0x75443d, 0x943f65, 0x3a588c, 0x8c3f89, 0xa7484c, 0x4e6365, 0x70614e, 0x5e7257, 0x9762f6, 0x548a5c, 0x7f72f9, 0xf071a7, 0xe174f2, 0xf77782, 0x52aaa5, 0x86a656, 0xf78369, 0xb394fc, 0x74aff0, 0xf688e6, 0x57c764, 0x80b8d2, 0x52d196, 0xe4b15d, 0xefb6fd, 0x60eaec, 0x70f253, 0xcee14b];

        for color in 0..=0xffffff
        {
            let mut least_delta = (0x0, (0x100,0x100,0x100));
            for palette_color in 0..=31
            {
                let mut color_delta = (0x100,0x100,0x100);
                color_delta.0 = (((color & 0xff0000) >> 16) -  ((color_palette[palette_color as usize] & 0xff0000) >> 16))  .abs();
                color_delta.1 = (((color & 0x00ff00) >> 8) -   ((color_palette[palette_color as usize] & 0x00ff00) >> 8))   .abs();
                color_delta.2 = (((color & 0x0000ff)) -        ((color_palette[palette_color as usize] & 0x0000ff)))        .abs();

                if color_delta.0 + color_delta.1 + color_delta.2 < least_delta.1.0 + least_delta.1.1 + least_delta.1.2
                {
                    least_delta.0 = palette_color;
                    least_delta.1 = color_delta;
                }
            }

            self.colormap_table[color as usize] = color_palette[least_delta.0] as usize;
        }
    }

    pub fn get_nearest_color(&mut self, hex: usize) -> (u8, u8, u8)
    {
        if hex <= 0x00ffffff
        {
            let color = self.colormap_table[hex];
            (((color & 0xff0000) >> 16) as u8,((color & 0x00ff00) >> 8 ) as u8, ((color & 0x0000ff)) as u8)
        }
        else
        {
            (0, 0, 0)
        }
    }

    pub fn get_vram_pixel(&mut self, mut pixel: usize) -> (u8, u8, u8)
    {
        pixel = pixel.saturating_add(self.vram_offset);
        self.get_nearest_color(self.memory[pixel as usize] as usize)
    }

    pub fn get_palette_nearest_color(&mut self, hue: u16, value: u8) -> (u8, u8, u8)
    {
        let color_map: [[u16; 16]; 16] =
        [
            [ 0x534, 0x534, 0x765, 0x675, 0x675, 0x675, 0x675, 0x566, 0x426, 0x426, 0x426, 0x426, 0x426, 0x426, 0x534, 0x534 ],
            [ 0x744, 0x744, 0x765, 0x596, 0x596, 0x596, 0x596, 0x566, 0x469, 0x426, 0x438, 0x438, 0x426, 0x426, 0x534, 0x744 ],
            [ 0xa55, 0x744, 0x8a5, 0x8a5, 0x596, 0x596, 0x596, 0x596, 0x469, 0x438, 0x438, 0x438, 0x438, 0x949, 0x946, 0xa55 ],
            [ 0xa55, 0x744, 0x8a5, 0x8a5, 0x5c6, 0x5c6, 0x596, 0x5ba, 0x469, 0x438, 0x54c, 0x54c, 0x438, 0x949, 0x946, 0xa55 ],
            [ 0xa55, 0xeb6, 0x8a5, 0x8a5, 0x5c6, 0x5c6, 0x5c6, 0x5ba, 0x469, 0x438, 0x54c, 0x54c, 0x54c, 0x949, 0x946, 0xa55 ],
            [ 0xa55, 0xeb6, 0xde5, 0x8a5, 0x5c6, 0x5c6, 0x5c6, 0x5ba, 0x469, 0x438, 0x54c, 0x54c, 0x54c, 0x949, 0x946, 0xa55 ],
            [ 0xf87, 0xeb6, 0xde5, 0x5c6, 0x5c6, 0x5c6, 0x5c6, 0x5ba, 0x469, 0x54c, 0x54c, 0x54c, 0x96f, 0x949, 0x946, 0xa55 ],
            [ 0xf87, 0xeb6, 0xde5, 0xde5, 0x7f5, 0x7f5, 0x5c6, 0x5ba, 0x7bf, 0x54c, 0x54c, 0x54c, 0x96f, 0xe7f, 0xf7a, 0xa55 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x5c6, 0x5d9, 0x7bf, 0x54c, 0x54c, 0x54c, 0x96f, 0xe7f, 0xf7a, 0xa55 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x5c6, 0x5d9, 0x7bf, 0x54c, 0x54c, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x5c6, 0x5d9, 0x7bf, 0x54c, 0x54c, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x5c6, 0x5d9, 0x7bf, 0x54c, 0x96f, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x7f5, 0x5d9, 0x7bf, 0x54c, 0x96f, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x7f5, 0x5d9, 0x7bf, 0x54c, 0x96f, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x7f5, 0x6ff, 0x7bf, 0x87f, 0x96f, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],
            [ 0xf87, 0xeb6, 0xde5, 0x7f5, 0x7f5, 0x7f5, 0x7f5, 0x6ff, 0x7bf, 0x87f, 0x96f, 0x96f, 0x96f, 0xe7f, 0xf7a, 0xf87 ],

        ];

        let corrected_hue = ((hue * 10 / 14) as u8) >> 4;
        let corrected_value = (value & 0xf0) >> 0x4;
        (
            ((color_map[corrected_value as usize][corrected_hue as usize] & 0x0f00) >> 4) as u8,
            ((color_map[corrected_value as usize][corrected_hue as usize] & 0x00f0)) as u8,
            ((color_map[corrected_value as usize][corrected_hue as usize] & 0x000f) << 4) as u8
        )
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, use_color: bool)
    {
        for pixel in 0..0x4000
        {
            let color = self.memory[self.vram_offset as usize + pixel as usize].to_be_bytes();
            match use_color
            {
                false => canvas.set_draw_color(self.get_palette_nearest_color(300, (color[1] as u16 + color[2] as u16 + color[3] as u16 / 3) as u8)),
                true => canvas.set_draw_color(self.get_vram_pixel(pixel)),
            }

            let _ = canvas.fill_rect(
                sdl2::rect::Rect::new(
                    10+((pixel & 0x007f) * self.display_scale_by) as i32,
                    10+(((pixel & 0xff80) >> 7) * self.display_scale_by) as i32,
                    u32::from_usize(self.display_scale_by)[0],
                    u32::from_usize(self.display_scale_by)[0]
                )
            ).is_ok();
        }
    }

    pub fn flip_vram_vertically(&mut self)
    {
        let mut vram_copy = vec![0x0; self.display_size as usize];
        for pixel in 0..self.display_size
        {
            vram_copy[pixel as usize] =
                self.memory[self.vram_offset as usize + pixel as usize];
        }
        for y in 1..129
        {
            for x in 0..128
            {
                self.memory[0x3ffffff - (y * 128) + x] = vram_copy[x + ((y - 1) * 128)];
            }
        }
    }

}

pub trait FromUsize
{
    fn from_usize(number: usize) -> [u32; 2];
}


impl FromUsize for u32
{
    fn from_usize( number: usize ) -> [u32; 2]
    {
        let number = number.to_be_bytes();
        match usize::BITS
        {
            32 =>
            {
                [
                    u32::from_be_bytes([number[0], number[1], number[2], number[3]]),
                    0
                ]
            }
            64 =>
            {
                [
                    u32::from_be_bytes([number[4], number[5], number[6], number[7]]),
                    u32::from_be_bytes([number[0], number[1], number[2], number[3]])
                ]
            }
            _ => [0,0]
        }
    }
}

fn main()
{
    let mut computer = NewComputer
    {
        memory:             vec![0x0; 0x4000000],
        program_position:   0x0,
        buffer:             [0x0; 0x4],
        colormap_table:     vec![0x0; 16777216],
        display_scale_by:   6,
        display_size:       128 * 128,
        vram_offset:        0x03ffc000,

        input_offset:       0x03ffbfff,
        character_table_offset: 0x003ffbbc,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("X4C", 128*u32::from_usize(computer.display_scale_by)[0]+20, 128*u32::from_usize(computer.display_scale_by)[0]+20)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(computer.get_palette_nearest_color(0x0, 0x0));
    canvas.clear();
    canvas.present();

    for pixel in computer.vram_offset..=0x3ffffff
    {
        match ((pixel - computer.vram_offset) + (pixel - computer.vram_offset) / 128) % 2
        {
            0 => computer.memory[pixel as usize] = 0xffffff,
            1 => computer.memory[pixel as usize] = 0x000000,
            _ => ()
        }
    }

    let _ = computer.draw(&mut canvas, false);
    canvas.present();
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let mut event_pump = sdl_context.event_pump().unwrap();


    for pixel in computer.vram_offset..=0x3ffffff
    {
        match ((pixel - computer.vram_offset) + (pixel - computer.vram_offset) / 128) % 2
        {
            0 => computer.memory[pixel as usize] = 0xffffff,
            1 => computer.memory[pixel as usize] = 0x000000,
            _ => ()
        }

    }

    let boot_pic = std::fs::File::open("boot.bmp");
    if boot_pic.is_ok()
    {
        let mut boot_pic = boot_pic.unwrap();
        let mut boot_pic_bytes: [u8; 65536+138] = [0; 65536+138];
        let boot_pic_bytes_state = std::io::Read::read(&mut boot_pic, &mut boot_pic_bytes);
        if boot_pic_bytes_state.is_ok()
        {
            let boot_pic_image_start = usize::from_le_bytes( [boot_pic_bytes[10],boot_pic_bytes[11], boot_pic_bytes[12], boot_pic_bytes[13], 0x0, 0x0, 0x0, 0x0] );
            for byte in 0..boot_pic_bytes.len().saturating_sub(boot_pic_image_start as usize).div_euclid(4)
            {
                let boot_byte = boot_pic_image_start + byte.saturating_mul(4) as usize;
                computer.memory[computer.vram_offset as usize + byte as usize] =
                    usize::from_be_bytes(
                        [
                            0x0,
                            boot_pic_bytes[boot_byte as usize],
                            boot_pic_bytes[boot_byte as usize+1],
                            boot_pic_bytes[boot_byte as usize+2],
                            0x0,
                            0x0,
                            0x0,
                            0x0
                        ]
                    );
            }
        }
        computer.flip_vram_vertically();
    }

    canvas.clear();

    let _ = computer.draw(&mut canvas, false);

    canvas.present();
    computer.calculate_color_corrections();

    for pixel in computer.vram_offset..=0x3ffffff
    {
        computer.memory[pixel as usize] = 0x303030;
    }

    computer.memory[0] = 0x20000002;
    computer.memory[1] = 0x10000000;
    computer.memory[2] = 0xffffffff;

    let font: [usize; 41] =
    [
        0b011101000110001111111000110001, // a
        0b111101000110001111101000111110, // b
        0b111111000010000100001000011111, // c
        0b111101000110001100011000111110, // d
        0b111111000010000111111000011111, // e
        0b111111000010000111111000010000, // f
        0b111111000010000100111000111111, // g
        0b100011000110001111111000110001, // h
        0b111110010000100001000010011111, // i
        0b111110010000100001000010011100, // j
        0b100011011011000110001011010001, // k
        0b100001000010000100001000011111, // l
        0b110111010110101101011010110101, // m
        0b100011100110101100111000110001, // n
        0b011101000110001100011000101110, // o
        0b111101000110001111101000010000, // p
        0b011101000110001100011001001101, // q
        0b111101000110010111001001010001, // r
        0b011111000010000011100000111110, // s
        0b111110010000100001000010000100, // t
        0b100011000110001100011000101110, // u
        0b100011000110001010100101000100, // v
        0b100011010110101101011111101010, // w
        0b100011000101010001000101010001, // x
        0b100011000101010001000010000100, // y
        0b111110000100010011001000011111, // z
        0b011101001110101101011100101110, // 0
        0b001001110000100001000010011111, // 1
        0b111100000100001001101100011111, // 2
        0b111100000100001011100000111110, // 3
        0b001100101001010100101111100010, // 4
        0b111111000010000111100000111110, // 5
        0b011101000010000111101000101110, // 6
        0b111110000100010000100010000100, // 7
        0b011101000110001011101000101110, // 8
        0b011101000110001011110000111110, // 9
        0b000000000000000000000000000100, // .
        0b000000001000100010000010000010, // <
        0b000000100000100000100010001000, // >
        0b000000101011111010101111101010, // #
        0b001000010000100001000000000100, // !
    ];

    for character in 0..41
    {
        computer.memory[computer.character_table_offset as usize + character] = font[character];
    }

    let character_offset = computer.vram_offset + 128 + 1;
    let character = computer.memory[computer.character_table_offset as usize + 38];
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut mask: usize = 0x80000000;
    mask = mask >> 2;

    for iteration in 0..30
    {
        let bit = character & mask;
        mask = mask >> 1;
        if bit > 0
        {
            computer.memory[character_offset as usize + x as usize + (y as usize * 128)] = 0x00ffffff;
        }

        y = (iteration + 1) / 5;
        x = x.wrapping_add(1);
        if x == 5
        {
            x = 0;
        }
    }

    'running: loop
    {
        canvas.set_draw_color(computer.get_nearest_color(0x0));

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } =>
                {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F5), .. } =>
                {
                    let memory_file = std::fs::File::create("x4c-memory.hex");
                    if memory_file.is_ok()
                    {
                        let mut memory_file = memory_file.unwrap();
                        let mut file_data: Vec<u8> = vec![];
                        for address in 0..computer.memory.len()
                        {
                            let address_read = computer.memory[address].to_be_bytes();
                            file_data.append(&mut address_read.to_vec());
                        }
                        let _ = std::io::Write::write_all(&mut memory_file, &file_data);
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F6), .. } =>
                {
                    let memory_file = std::fs::File::create("colormap.hex");
                    if memory_file.is_ok()
                    {
                        let mut memory_file = memory_file.unwrap();
                        let mut file_data: Vec<u8> = vec![];
                        for address in 0..computer.colormap_table.len()
                        {
                            let address_read = computer.colormap_table[address].to_be_bytes();
                            file_data.append(&mut address_read.to_vec());
                        }
                        let _ = std::io::Write::write_all(&mut memory_file, &file_data);
                    }

                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F2), .. } =>
                {
                    computer.display_scale_by = computer.display_scale_by.wrapping_sub(1);
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F3), .. } =>
                {
                    computer.display_scale_by = computer.display_scale_by.wrapping_add(1);
                },
                sdl2::event::Event::KeyDown { keycode, .. } =>
                {
                    if keycode.is_some()
                    {
                        computer.memory[computer.input_offset as usize] = keycode.unwrap().into_i32() as usize;
                        println!("{:?}",keycode);
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(computer.get_nearest_color(0xff0000));
        let _ = canvas.draw_rect(
            sdl2::rect::Rect::new(
                9,
                9,
                u32::from_usize(computer.display_scale_by)[0] * 128+2,
                u32::from_usize(computer.display_scale_by)[0] * 128+2
            )
        ).is_ok();
        computer.process_instruction();

        let _ = computer.draw(&mut canvas, true);
        canvas.present();
    }
}
