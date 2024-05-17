#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Bit::*;
use common::*;
pub fn FPSCR_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1850: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        gs_1850: (),
    }
    let fn_state = FunctionState {
        gs_1850,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #15384u : u32
        let s_0_0: u32 = 15384;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #20696u : u32
        let s_0_2: u32 = 20696;
        // D s_0_3: read-reg s_0_2:u64
        let s_0_3: u64 = {
            let value = state.read_register::<u64>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #7s : i
        let s_0_4: i128 = 7;
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // D s_0_6: cast zx s_0_3 -> bv
        let s_0_6: Bits = Bits::new(s_0_3 as u128, 64u16);
        // D s_0_7: bit-extract s_0_6 s_0_4 s_0_5
        let s_0_7: Bits = (Bits::new(
            ((s_0_6) >> (s_0_4)).value(),
            u16::try_from(s_0_5).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: bool = ((s_0_7.value()) != 0);
        // D s_0_9: call Bit(s_0_8)
        let s_0_9: bool = Bit(state, tracer, s_0_8);
        // N s_0_10: return s_0_1
        return s_0_1;
    }
}
