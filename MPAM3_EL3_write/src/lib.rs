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
pub fn MPAM3_EL3_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // D s_0_2: read-var r.0:struct
        let s_0_2: u64 = fn_state.r._0;
        // C s_0_3: const #63s : i
        let s_0_3: i128 = 63;
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 64u16);
        // D s_0_6: bit-extract s_0_5 s_0_3 s_0_4
        let s_0_6: Bits = (Bits::new(
            ((s_0_5) >> (s_0_3)).value(),
            u16::try_from(s_0_4).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u8
        let s_0_7: bool = ((s_0_6.value()) != 0);
        // D s_0_8: call Bit(s_0_7)
        let s_0_8: bool = Bit(state, tracer, s_0_7);
        // C s_0_9: const #90504u : u32
        let s_0_9: u32 = 90504;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // C s_0_11: const #90504u : u32
        let s_0_11: u32 = 90504;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // C s_0_13: const #12792u : u32
        let s_0_13: u32 = 12792;
        // N s_0_14: write-reg s_0_13 <= s_0_0
        let s_0_14: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize, s_0_0);
            tracer.write_register(s_0_13 as isize, s_0_0);
        };
        // N s_0_15: return
        return;
    }
}
