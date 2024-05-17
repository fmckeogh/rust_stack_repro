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
use R_read::*;
use R_set::*;
use Shift::*;
use SignedSatQ::*;
use common::*;
pub fn execute_aarch32_instrs_SSAT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    saturate_to: i64,
    shift_n: i128,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_347392: ProductTypef506aa96a892fe52,
        d: i64,
        n: i64,
        saturate_to: i64,
        shift_n: i128,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        d,
        n,
        saturate_to,
        shift_n,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var saturate_to:i64
        let s_0_0: i64 = fn_state.saturate_to;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: call R_read(s_0_4)
        let s_0_5: u32 = R_read(state, tracer, s_0_4);
        // C s_0_6: const #16971u : u32
        let s_0_6: u32 = 16971;
        // D s_0_7: read-reg s_0_6:u8
        let s_0_7: bool = {
            let value = state.read_register::<bool>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: cast zx s_0_5 -> bv
        let s_0_8: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_9: read-var shift_t:u32
        let s_0_9: u32 = fn_state.shift_t;
        // D s_0_10: read-var shift_n:i
        let s_0_10: i128 = fn_state.shift_n;
        // D s_0_11: call Shift(s_0_8, s_0_9, s_0_10, s_0_7)
        let s_0_11: Bits = Shift(state, tracer, s_0_8, s_0_9, s_0_10, s_0_7);
        // D s_0_12: cast reint s_0_11 -> u32
        let s_0_12: u32 = (s_0_11.value() as u32);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 32u16);
        // D s_0_14: cast sx s_0_13 -> i
        let s_0_14: i128 = {
            let sign_bit = s_0_13.length() - 1;
            let mut result = s_0_13.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: cast zx s_0_2 -> i
        let s_0_16: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // D s_0_18: cast zx s_0_15 -> i
        let s_0_18: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_19: cast zx s_0_17 -> i
        let s_0_19: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_20: call SignedSatQ(s_0_18, s_0_19)
        let s_0_20: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_18,
            s_0_19,
        );
        // D s_0_21: write-var ga#347392 <= s_0_20
        fn_state.ga_347392 = s_0_20;
        // D s_0_22: read-var ga#347392.0:struct
        let s_0_22: Bits = fn_state.ga_347392._0;
        // D s_0_23: read-var ga#347392.1:struct
        let s_0_23: bool = fn_state.ga_347392._1;
        // C s_0_24: const #32s : i
        let s_0_24: i128 = 32;
        // D s_0_25: bits-cast sx s_0_22 -> bv length s_0_24
        let s_0_25: Bits = s_0_22.sign_extend(s_0_24);
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: read-var d:i64
        let s_0_27: i64 = fn_state.d;
        // D s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_29: call R_set(s_0_28, s_0_26)
        let s_0_29: () = R_set(state, tracer, s_0_28, s_0_26);
        // N s_0_30: branch s_0_23 b2 b1
        if s_0_23 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: const #16988u : u32
        let s_2_1: u32 = 16988;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<bool>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // N s_2_3: return
        return;
    }
}
