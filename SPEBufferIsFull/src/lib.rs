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
use u_get_PMSIDR_EL1_Type_MaxSize::*;
use u_get_PMBLIMITR_EL1_Type_LIMIT::*;
use Zeros::*;
use common::*;
pub fn SPEBufferIsFull<T: Tracer>(state: &mut State, tracer: &T, gs_25711: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_25711: (),
    }
    let fn_state = FunctionState {
        gs_25711,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #20480u : u32
        let s_0_0: u32 = 20480;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMBLIMITR_EL1_Type_LIMIT(s_0_1)
        let s_0_2: u64 = u_get_PMBLIMITR_EL1_Type_LIMIT(state, tracer, s_0_1);
        // C s_0_3: const #12s : i
        let s_0_3: i128 = 12;
        // S s_0_4: call Zeros(s_0_3)
        let s_0_4: Bits = Zeros(state, tracer, s_0_3);
        // S s_0_5: cast reint s_0_4 -> u12
        let s_0_5: u16 = (s_0_4.value() as u16);
        // D s_0_6: cast zx s_0_2 -> bv
        let s_0_6: Bits = Bits::new(s_0_2 as u128, 52u16);
        // S s_0_7: cast zx s_0_5 -> bv
        let s_0_7: Bits = Bits::new(s_0_5 as u128, 12u16);
        // D s_0_8: cast reint s_0_6 -> u128
        let s_0_8: u128 = (s_0_6.value() as u128);
        // D s_0_9: size-of s_0_6
        let s_0_9: u16 = s_0_6.length();
        // S s_0_10: cast reint s_0_7 -> u128
        let s_0_10: u128 = (s_0_7.value() as u128);
        // D s_0_11: size-of s_0_7
        let s_0_11: u16 = s_0_7.length();
        // D s_0_12: lsl s_0_8 s_0_11
        let s_0_12: u128 = s_0_8 << s_0_11;
        // D s_0_13: or s_0_12 s_0_10
        let s_0_13: u128 = ((s_0_12) | (s_0_10));
        // D s_0_14: add s_0_9 s_0_11
        let s_0_14: u16 = (s_0_9 + s_0_11);
        // D s_0_15: create-bits s_0_13 s_0_14
        let s_0_15: Bits = Bits::new(s_0_13, s_0_14);
        // D s_0_16: cast reint s_0_15 -> u64
        let s_0_16: u64 = (s_0_15.value() as u64);
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (s_0_17.value() as i128);
        // C s_0_19: const #90512u : u32
        let s_0_19: u32 = 90512;
        // D s_0_20: read-reg s_0_19:u64
        let s_0_20: u64 = {
            let value = state.read_register::<u64>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 64u16);
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (s_0_21.value() as i128);
        // C s_0_23: const #14736u : u32
        let s_0_23: u32 = 14736;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_PMSIDR_EL1_Type_MaxSize(s_0_24)
        let s_0_25: u8 = u_get_PMSIDR_EL1_Type_MaxSize(state, tracer, s_0_24);
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 4u16);
        // D s_0_27: cast zx s_0_26 -> i
        let s_0_27: i128 = (s_0_26.value() as i128);
        // D s_0_28: cast reint s_0_27 -> i64
        let s_0_28: i64 = (s_0_27 as i64);
        // C s_0_29: const #1s : i
        let s_0_29: i128 = 1;
        // D s_0_30: cast zx s_0_28 -> i
        let s_0_30: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_31: lsl s_0_29 s_0_30
        let s_0_31: i128 = s_0_29 << s_0_30;
        // D s_0_32: sub s_0_18 s_0_31
        let s_0_32: i128 = ((s_0_18) - (s_0_31));
        // D s_0_33: cmp-gt s_0_22 s_0_32
        let s_0_33: bool = ((s_0_22) > (s_0_32));
        // N s_0_34: return s_0_33
        return s_0_33;
    }
}
