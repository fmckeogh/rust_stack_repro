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
use ZeroUnsignedSatQ::*;
use u__id::*;
use R_read::*;
use Shift::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_USAT_Op_A_txt<T: Tracer>(
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
        gs_889105: ProductTypef506aa96a892fe52,
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
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #16971u : u32
        let s_0_3: u32 = 16971;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: read-var shift_t:u32
        let s_0_6: u32 = fn_state.shift_t;
        // D s_0_7: read-var shift_n:i
        let s_0_7: i128 = fn_state.shift_n;
        // D s_0_8: call Shift(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: Bits = Shift(state, tracer, s_0_5, s_0_6, s_0_7, s_0_4);
        // D s_0_9: cast reint s_0_8 -> u32
        let s_0_9: u32 = (s_0_8.value() as u32);
        // D s_0_10: read-var saturate_to:i64
        let s_0_10: i64 = fn_state.saturate_to;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call __id(s_0_11)
        let s_0_12: i128 = u__id(state, tracer, s_0_11);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // C s_0_14: const #1s : i
        let s_0_14: i128 = 1;
        // D s_0_15: cast zx s_0_13 -> i
        let s_0_15: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_16: sub s_0_15 s_0_14
        let s_0_16: i128 = ((s_0_15) - (s_0_14));
        // D s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // C s_0_18: const #0s : i
        let s_0_18: i128 = 0;
        // D s_0_19: cast zx s_0_17 -> i
        let s_0_19: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_20: cmp-le s_0_18 s_0_19
        let s_0_20: bool = ((s_0_18) <= (s_0_19));
        // N s_0_21: assert s_0_20
        let s_0_21: () = assert!(s_0_20);
        // D s_0_22: cast zx s_0_9 -> bv
        let s_0_22: Bits = Bits::new(s_0_9 as u128, 32u16);
        // D s_0_23: cast sx s_0_22 -> i
        let s_0_23: i128 = {
            let sign_bit = s_0_22.length() - 1;
            let mut result = s_0_22.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_24: cast reint s_0_23 -> i64
        let s_0_24: i64 = (s_0_23 as i64);
        // C s_0_25: const #32s : i64
        let s_0_25: i64 = 32;
        // D s_0_26: cast zx s_0_24 -> i
        let s_0_26: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_27: read-var saturate_to:i64
        let s_0_27: i64 = fn_state.saturate_to;
        // D s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (i128::try_from(s_0_27).unwrap());
        // C s_0_29: cast zx s_0_25 -> i
        let s_0_29: i128 = (i128::try_from(s_0_25).unwrap());
        // D s_0_30: call ZeroUnsignedSatQ(s_0_26, s_0_28, s_0_29)
        let s_0_30: ProductTypef506aa96a892fe52 = ZeroUnsignedSatQ(
            state,
            tracer,
            s_0_26,
            s_0_28,
            s_0_29,
        );
        // D s_0_31: write-var gs#889105 <= s_0_30
        fn_state.gs_889105 = s_0_30;
        // D s_0_32: read-var gs#889105.0:struct
        let s_0_32: Bits = fn_state.gs_889105._0;
        // D s_0_33: cast reint s_0_32 -> u32
        let s_0_33: u32 = (s_0_32.value() as u32);
        // D s_0_34: read-var gs#889105.1:struct
        let s_0_34: bool = fn_state.gs_889105._1;
        // D s_0_35: read-var d:i64
        let s_0_35: i64 = fn_state.d;
        // D s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_37: call R_set(s_0_36, s_0_33)
        let s_0_37: () = R_set(state, tracer, s_0_36, s_0_33);
        // N s_0_38: branch s_0_34 b2 b1
        if s_0_34 {
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
