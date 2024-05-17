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
use u_get_SPMSELR_EL0_Type_SYSPMUSEL::*;
use common::*;
pub fn u__get_selected_SPMACCESSR_EL1_field<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_32776: (),
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_32776: (),
    }
    let fn_state = FunctionState {
        gs_32776,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #16552u : u32
        let s_0_0: u32 = 16552;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_0_1)
        let s_0_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 6u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // C s_0_6: const #2s : i
        let s_0_6: i128 = 2;
        // D s_0_7: cast zx s_0_5 -> i
        let s_0_7: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_8: mul s_0_7 s_0_6
        let s_0_8: i128 = ((s_0_7) * (s_0_6));
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // C s_0_10: const #11944u : u32
        let s_0_10: u32 = 11944;
        // D s_0_11: read-reg s_0_10:u64
        let s_0_11: u64 = {
            let value = state.read_register::<u64>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #2s : i
        let s_0_12: i128 = 2;
        // D s_0_13: cast zx s_0_11 -> bv
        let s_0_13: Bits = Bits::new(s_0_11 as u128, 64u16);
        // D s_0_14: cast zx s_0_9 -> i
        let s_0_14: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_15: bit-extract s_0_13 s_0_14 s_0_12
        let s_0_15: Bits = (Bits::new(
            ((s_0_13) >> (s_0_14)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // N s_0_17: return s_0_16
        return s_0_16;
    }
}
