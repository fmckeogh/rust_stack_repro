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
use common::*;
pub fn execute_aarch32_instrs_PKH_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
    tbform: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_345398: u16,
        ga_345394: u16,
        ga_345393: u32,
        ga_345397: u32,
        operand2: u32,
        d: i64,
        m: i64,
        n: i64,
        shift_n: i128,
        shift_t: u32,
        tbform: bool,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        shift_n,
        shift_t,
        tbform,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
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
        // D s_0_10: write-var operand2 <= s_0_9
        fn_state.operand2 = s_0_9;
        // D s_0_11: read-var d:i64
        let s_0_11: i64 = fn_state.d;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: call R_read(s_0_12)
        let s_0_13: u32 = R_read(state, tracer, s_0_12);
        // D s_0_14: write-var ga#345393 <= s_0_13
        fn_state.ga_345393 = s_0_13;
        // D s_0_15: read-var tbform:u8
        let s_0_15: bool = fn_state.tbform;
        // N s_0_16: branch s_0_15 b6 b1
        if s_0_15 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // C s_1_3: const #0s : i
        let s_1_3: i128 = 0;
        // D s_1_4: cast zx s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 32u16);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // C s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // C s_1_7: const #15s : i
        let s_1_7: i128 = 15;
        // C s_1_8: add s_1_7 s_1_6
        let s_1_8: i128 = (s_1_7 + s_1_6);
        // D s_1_9: bit-extract s_1_4 s_1_3 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_4) >> (s_1_3)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u16
        let s_1_10: u16 = (s_1_9.value() as u16);
        // D s_1_11: write-var ga#345394 <= s_1_10
        fn_state.ga_345394 = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var ga#345393:u32
        let s_2_1: u32 = fn_state.ga_345393;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // D s_2_3: read-var ga#345394:u16
        let s_2_3: u16 = fn_state.ga_345394;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 16u16);
        // C s_2_5: const #15s : i
        let s_2_5: i128 = 15;
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: lsl s_2_7 s_2_5
        let s_2_8: Bits = s_2_7 << s_2_5;
        // C s_2_9: sub s_2_8 s_2_7
        let s_2_9: Bits = ((s_2_8) - (s_2_7));
        // D s_2_10: and s_2_4 s_2_9
        let s_2_10: Bits = ((s_2_4) & (s_2_9));
        // D s_2_11: lsl s_2_10 s_2_0
        let s_2_11: Bits = s_2_10 << s_2_0;
        // C s_2_12: lsl s_2_9 s_2_0
        let s_2_12: Bits = s_2_9 << s_2_0;
        // C s_2_13: cmpl s_2_12
        let s_2_13: Bits = !s_2_12;
        // D s_2_14: and s_2_2 s_2_13
        let s_2_14: Bits = ((s_2_2) & (s_2_13));
        // D s_2_15: or s_2_14 s_2_11
        let s_2_15: Bits = ((s_2_14) | (s_2_11));
        // D s_2_16: cast reint s_2_15 -> u32
        let s_2_16: u32 = (s_2_15.value() as u32);
        // D s_2_17: read-var d:i64
        let s_2_17: i64 = fn_state.d;
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_19: call R_set(s_2_18, s_2_16)
        let s_2_19: () = R_set(state, tracer, s_2_18, s_2_16);
        // D s_2_20: read-var d:i64
        let s_2_20: i64 = fn_state.d;
        // D s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_22: call R_read(s_2_21)
        let s_2_22: u32 = R_read(state, tracer, s_2_21);
        // D s_2_23: write-var ga#345397 <= s_2_22
        fn_state.ga_345397 = s_2_22;
        // D s_2_24: read-var tbform:u8
        let s_2_24: bool = fn_state.tbform;
        // N s_2_25: branch s_2_24 b5 b3
        if s_2_24 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: read-var operand2:u32
        let s_3_1: u32 = fn_state.operand2;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 32u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #15s : i
        let s_3_5: i128 = 15;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u16
        let s_3_8: u16 = (s_3_7.value() as u16);
        // D s_3_9: write-var ga#345398 <= s_3_8
        fn_state.ga_345398 = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var ga#345397:u32
        let s_4_1: u32 = fn_state.ga_345397;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_3: read-var ga#345398:u16
        let s_4_3: u16 = fn_state.ga_345398;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 16u16);
        // C s_4_5: const #15s : i
        let s_4_5: i128 = 15;
        // C s_4_6: const #1u : u64
        let s_4_6: u64 = 1;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 64u16);
        // C s_4_8: lsl s_4_7 s_4_5
        let s_4_8: Bits = s_4_7 << s_4_5;
        // C s_4_9: sub s_4_8 s_4_7
        let s_4_9: Bits = ((s_4_8) - (s_4_7));
        // D s_4_10: and s_4_4 s_4_9
        let s_4_10: Bits = ((s_4_4) & (s_4_9));
        // D s_4_11: lsl s_4_10 s_4_0
        let s_4_11: Bits = s_4_10 << s_4_0;
        // C s_4_12: lsl s_4_9 s_4_0
        let s_4_12: Bits = s_4_9 << s_4_0;
        // C s_4_13: cmpl s_4_12
        let s_4_13: Bits = !s_4_12;
        // D s_4_14: and s_4_2 s_4_13
        let s_4_14: Bits = ((s_4_2) & (s_4_13));
        // D s_4_15: or s_4_14 s_4_11
        let s_4_15: Bits = ((s_4_14) | (s_4_11));
        // D s_4_16: cast reint s_4_15 -> u32
        let s_4_16: u32 = (s_4_15.value() as u32);
        // D s_4_17: read-var d:i64
        let s_4_17: i64 = fn_state.d;
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: call R_set(s_4_18, s_4_16)
        let s_4_19: () = R_set(state, tracer, s_4_18, s_4_16);
        // N s_4_20: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call R_read(s_5_1)
        let s_5_2: u32 = R_read(state, tracer, s_5_1);
        // C s_5_3: const #16s : i
        let s_5_3: i128 = 16;
        // D s_5_4: cast zx s_5_2 -> bv
        let s_5_4: Bits = Bits::new(s_5_2 as u128, 32u16);
        // C s_5_5: const #1s : i64
        let s_5_5: i64 = 1;
        // C s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // C s_5_7: const #15s : i
        let s_5_7: i128 = 15;
        // C s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: bit-extract s_5_4 s_5_3 s_5_8
        let s_5_9: Bits = (Bits::new(
            ((s_5_4) >> (s_5_3)).value(),
            u16::try_from(s_5_8).unwrap(),
        ));
        // D s_5_10: cast reint s_5_9 -> u16
        let s_5_10: u16 = (s_5_9.value() as u16);
        // D s_5_11: write-var ga#345398 <= s_5_10
        fn_state.ga_345398 = s_5_10;
        // N s_5_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var operand2:u32
        let s_6_1: u32 = fn_state.operand2;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: const #15s : i
        let s_6_5: i128 = 15;
        // C s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: bit-extract s_6_2 s_6_0 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u16
        let s_6_8: u16 = (s_6_7.value() as u16);
        // D s_6_9: write-var ga#345394 <= s_6_8
        fn_state.ga_345394 = s_6_8;
        // N s_6_10: jump b2
        return block_2(state, tracer, fn_state);
    }
}
