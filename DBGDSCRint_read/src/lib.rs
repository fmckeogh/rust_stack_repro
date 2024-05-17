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
pub fn DBGDSCRint_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_9386: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        gs_9386: (),
    }
    let fn_state = FunctionState {
        gs_9386,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #23040u : u32
        let s_0_0: u32 = 23040;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #104648u : u32
        let s_0_2: u32 = 104648;
        // D s_0_3: read-reg s_0_2:u64
        let s_0_3: u64 = {
            let value = state.read_register::<u64>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #15s : i
        let s_0_4: i128 = 15;
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
        // C s_0_10: const #104648u : u32
        let s_0_10: u32 = 104648;
        // D s_0_11: read-reg s_0_10:u64
        let s_0_11: u64 = {
            let value = state.read_register::<u64>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #12s : i
        let s_0_12: i128 = 12;
        // C s_0_13: const #1s : i
        let s_0_13: i128 = 1;
        // D s_0_14: cast zx s_0_11 -> bv
        let s_0_14: Bits = Bits::new(s_0_11 as u128, 64u16);
        // D s_0_15: bit-extract s_0_14 s_0_12 s_0_13
        let s_0_15: Bits = (Bits::new(
            ((s_0_14) >> (s_0_12)).value(),
            u16::try_from(s_0_13).unwrap(),
        ));
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: bool = ((s_0_15.value()) != 0);
        // D s_0_17: call Bit(s_0_16)
        let s_0_17: bool = Bit(state, tracer, s_0_16);
        // N s_0_18: return s_0_1
        return s_0_1;
    }
}
