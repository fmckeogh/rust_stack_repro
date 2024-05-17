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
use R_set::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_SEL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_346490: u8,
        ga_346481: u8,
        ga_346480: u32,
        ga_346499: u8,
        ga_346498: u32,
        ga_346471: u32,
        ga_346489: u32,
        ga_346472: u8,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var d:i64
        let s_0_0: i64 = fn_state.d;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: write-var ga#346471 <= s_0_2
        fn_state.ga_346471 = s_0_2;
        // C s_0_4: const #16978u : u32
        let s_0_4: u32 = 16978;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // D s_0_7: cast zx s_0_5 -> bv
        let s_0_7: Bits = Bits::new(s_0_5 as u128, 4u16);
        // C s_0_8: const #1u : u64
        let s_0_8: u64 = 1;
        // D s_0_9: bit-extract s_0_7 s_0_6 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_7) >> (s_0_6)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: bool = ((s_0_9.value()) != 0);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // C s_0_12: const #0u : u64
        let s_0_12: u64 = 0;
        // D s_0_13: cast zx s_0_10 -> u64
        let s_0_13: u64 = (s_0_10 as u64);
        // C s_0_14: const #1u : u64
        let s_0_14: u64 = 1;
        // D s_0_15: and s_0_13 s_0_14
        let s_0_15: u64 = ((s_0_13) & (s_0_14));
        // D s_0_16: cmp-eq s_0_15 s_0_14
        let s_0_16: bool = ((s_0_15) == (s_0_14));
        // D s_0_17: lsl s_0_13 s_0_11
        let s_0_17: u64 = s_0_13 << s_0_11;
        // D s_0_18: or s_0_12 s_0_17
        let s_0_18: u64 = ((s_0_12) | (s_0_17));
        // D s_0_19: cmpl s_0_17
        let s_0_19: u64 = !s_0_17;
        // D s_0_20: and s_0_12 s_0_19
        let s_0_20: u64 = ((s_0_12) & (s_0_19));
        // D s_0_21: select s_0_16 s_0_18 s_0_20
        let s_0_21: u64 = if s_0_16 { s_0_18 } else { s_0_20 };
        // D s_0_22: cast trunc s_0_21 -> u8
        let s_0_22: bool = ((s_0_21) != 0);
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // C s_0_24: const #1u : u8
        let s_0_24: bool = true;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 1u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b12 b1
        if s_0_26 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
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
        // C s_1_7: const #7s : i
        let s_1_7: i128 = 7;
        // C s_1_8: add s_1_7 s_1_6
        let s_1_8: i128 = (s_1_7 + s_1_6);
        // D s_1_9: bit-extract s_1_4 s_1_3 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_4) >> (s_1_3)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u8
        let s_1_10: u8 = (s_1_9.value() as u8);
        // D s_1_11: write-var ga#346472 <= s_1_10
        fn_state.ga_346472 = s_1_10;
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
        // D s_2_1: read-var ga#346471:u32
        let s_2_1: u32 = fn_state.ga_346471;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // D s_2_3: read-var ga#346472:u8
        let s_2_3: u8 = fn_state.ga_346472;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 8u16);
        // C s_2_5: const #7s : i
        let s_2_5: i128 = 7;
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
        // D s_2_23: write-var ga#346480 <= s_2_22
        fn_state.ga_346480 = s_2_22;
        // C s_2_24: const #16978u : u32
        let s_2_24: u32 = 16978;
        // D s_2_25: read-reg s_2_24:u8
        let s_2_25: u8 = {
            let value = state.read_register::<u8>(s_2_24 as isize);
            tracer.read_register(s_2_24 as isize, value);
            value
        };
        // C s_2_26: const #1s : i
        let s_2_26: i128 = 1;
        // D s_2_27: cast zx s_2_25 -> bv
        let s_2_27: Bits = Bits::new(s_2_25 as u128, 4u16);
        // C s_2_28: const #1u : u64
        let s_2_28: u64 = 1;
        // D s_2_29: bit-extract s_2_27 s_2_26 s_2_28
        let s_2_29: Bits = (Bits::new(
            ((s_2_27) >> (s_2_26)).value(),
            u16::try_from(s_2_28).unwrap(),
        ));
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: bool = ((s_2_29.value()) != 0);
        // C s_2_31: const #0s : i
        let s_2_31: i128 = 0;
        // C s_2_32: const #0u : u64
        let s_2_32: u64 = 0;
        // D s_2_33: cast zx s_2_30 -> u64
        let s_2_33: u64 = (s_2_30 as u64);
        // C s_2_34: const #1u : u64
        let s_2_34: u64 = 1;
        // D s_2_35: and s_2_33 s_2_34
        let s_2_35: u64 = ((s_2_33) & (s_2_34));
        // D s_2_36: cmp-eq s_2_35 s_2_34
        let s_2_36: bool = ((s_2_35) == (s_2_34));
        // D s_2_37: lsl s_2_33 s_2_31
        let s_2_37: u64 = s_2_33 << s_2_31;
        // D s_2_38: or s_2_32 s_2_37
        let s_2_38: u64 = ((s_2_32) | (s_2_37));
        // D s_2_39: cmpl s_2_37
        let s_2_39: u64 = !s_2_37;
        // D s_2_40: and s_2_32 s_2_39
        let s_2_40: u64 = ((s_2_32) & (s_2_39));
        // D s_2_41: select s_2_36 s_2_38 s_2_40
        let s_2_41: u64 = if s_2_36 { s_2_38 } else { s_2_40 };
        // D s_2_42: cast trunc s_2_41 -> u8
        let s_2_42: bool = ((s_2_41) != 0);
        // D s_2_43: cast zx s_2_42 -> bv
        let s_2_43: Bits = Bits::new(s_2_42 as u128, 1u16);
        // C s_2_44: const #1u : u8
        let s_2_44: bool = true;
        // C s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 1u16);
        // D s_2_46: cmp-eq s_2_43 s_2_45
        let s_2_46: bool = ((s_2_43) == (s_2_45));
        // N s_2_47: branch s_2_46 b11 b3
        if s_2_46 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #8s : i
        let s_3_3: i128 = 8;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: const #7s : i
        let s_3_7: i128 = 7;
        // C s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: bit-extract s_3_4 s_3_3 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_4) >> (s_3_3)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u8
        let s_3_10: u8 = (s_3_9.value() as u8);
        // D s_3_11: write-var ga#346481 <= s_3_10
        fn_state.ga_346481 = s_3_10;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var ga#346480:u32
        let s_4_1: u32 = fn_state.ga_346480;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_3: read-var ga#346481:u8
        let s_4_3: u8 = fn_state.ga_346481;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 8u16);
        // C s_4_5: const #7s : i
        let s_4_5: i128 = 7;
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
        // D s_4_20: read-var d:i64
        let s_4_20: i64 = fn_state.d;
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_22: call R_read(s_4_21)
        let s_4_22: u32 = R_read(state, tracer, s_4_21);
        // D s_4_23: write-var ga#346489 <= s_4_22
        fn_state.ga_346489 = s_4_22;
        // C s_4_24: const #16978u : u32
        let s_4_24: u32 = 16978;
        // D s_4_25: read-reg s_4_24:u8
        let s_4_25: u8 = {
            let value = state.read_register::<u8>(s_4_24 as isize);
            tracer.read_register(s_4_24 as isize, value);
            value
        };
        // C s_4_26: const #2s : i
        let s_4_26: i128 = 2;
        // D s_4_27: cast zx s_4_25 -> bv
        let s_4_27: Bits = Bits::new(s_4_25 as u128, 4u16);
        // C s_4_28: const #1u : u64
        let s_4_28: u64 = 1;
        // D s_4_29: bit-extract s_4_27 s_4_26 s_4_28
        let s_4_29: Bits = (Bits::new(
            ((s_4_27) >> (s_4_26)).value(),
            u16::try_from(s_4_28).unwrap(),
        ));
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: bool = ((s_4_29.value()) != 0);
        // C s_4_31: const #0s : i
        let s_4_31: i128 = 0;
        // C s_4_32: const #0u : u64
        let s_4_32: u64 = 0;
        // D s_4_33: cast zx s_4_30 -> u64
        let s_4_33: u64 = (s_4_30 as u64);
        // C s_4_34: const #1u : u64
        let s_4_34: u64 = 1;
        // D s_4_35: and s_4_33 s_4_34
        let s_4_35: u64 = ((s_4_33) & (s_4_34));
        // D s_4_36: cmp-eq s_4_35 s_4_34
        let s_4_36: bool = ((s_4_35) == (s_4_34));
        // D s_4_37: lsl s_4_33 s_4_31
        let s_4_37: u64 = s_4_33 << s_4_31;
        // D s_4_38: or s_4_32 s_4_37
        let s_4_38: u64 = ((s_4_32) | (s_4_37));
        // D s_4_39: cmpl s_4_37
        let s_4_39: u64 = !s_4_37;
        // D s_4_40: and s_4_32 s_4_39
        let s_4_40: u64 = ((s_4_32) & (s_4_39));
        // D s_4_41: select s_4_36 s_4_38 s_4_40
        let s_4_41: u64 = if s_4_36 { s_4_38 } else { s_4_40 };
        // D s_4_42: cast trunc s_4_41 -> u8
        let s_4_42: bool = ((s_4_41) != 0);
        // D s_4_43: cast zx s_4_42 -> bv
        let s_4_43: Bits = Bits::new(s_4_42 as u128, 1u16);
        // C s_4_44: const #1u : u8
        let s_4_44: bool = true;
        // C s_4_45: cast zx s_4_44 -> bv
        let s_4_45: Bits = Bits::new(s_4_44 as u128, 1u16);
        // D s_4_46: cmp-eq s_4_43 s_4_45
        let s_4_46: bool = ((s_4_43) == (s_4_45));
        // N s_4_47: branch s_4_46 b10 b5
        if s_4_46 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
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
        // C s_5_7: const #7s : i
        let s_5_7: i128 = 7;
        // C s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: bit-extract s_5_4 s_5_3 s_5_8
        let s_5_9: Bits = (Bits::new(
            ((s_5_4) >> (s_5_3)).value(),
            u16::try_from(s_5_8).unwrap(),
        ));
        // D s_5_10: cast reint s_5_9 -> u8
        let s_5_10: u8 = (s_5_9.value() as u8);
        // D s_5_11: write-var ga#346490 <= s_5_10
        fn_state.ga_346490 = s_5_10;
        // N s_5_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // D s_6_1: read-var ga#346489:u32
        let s_6_1: u32 = fn_state.ga_346489;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_3: read-var ga#346490:u8
        let s_6_3: u8 = fn_state.ga_346490;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 8u16);
        // C s_6_5: const #7s : i
        let s_6_5: i128 = 7;
        // C s_6_6: const #1u : u64
        let s_6_6: u64 = 1;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 64u16);
        // C s_6_8: lsl s_6_7 s_6_5
        let s_6_8: Bits = s_6_7 << s_6_5;
        // C s_6_9: sub s_6_8 s_6_7
        let s_6_9: Bits = ((s_6_8) - (s_6_7));
        // D s_6_10: and s_6_4 s_6_9
        let s_6_10: Bits = ((s_6_4) & (s_6_9));
        // D s_6_11: lsl s_6_10 s_6_0
        let s_6_11: Bits = s_6_10 << s_6_0;
        // C s_6_12: lsl s_6_9 s_6_0
        let s_6_12: Bits = s_6_9 << s_6_0;
        // C s_6_13: cmpl s_6_12
        let s_6_13: Bits = !s_6_12;
        // D s_6_14: and s_6_2 s_6_13
        let s_6_14: Bits = ((s_6_2) & (s_6_13));
        // D s_6_15: or s_6_14 s_6_11
        let s_6_15: Bits = ((s_6_14) | (s_6_11));
        // D s_6_16: cast reint s_6_15 -> u32
        let s_6_16: u32 = (s_6_15.value() as u32);
        // D s_6_17: read-var d:i64
        let s_6_17: i64 = fn_state.d;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: call R_set(s_6_18, s_6_16)
        let s_6_19: () = R_set(state, tracer, s_6_18, s_6_16);
        // D s_6_20: read-var d:i64
        let s_6_20: i64 = fn_state.d;
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_22: call R_read(s_6_21)
        let s_6_22: u32 = R_read(state, tracer, s_6_21);
        // D s_6_23: write-var ga#346498 <= s_6_22
        fn_state.ga_346498 = s_6_22;
        // C s_6_24: const #16978u : u32
        let s_6_24: u32 = 16978;
        // D s_6_25: read-reg s_6_24:u8
        let s_6_25: u8 = {
            let value = state.read_register::<u8>(s_6_24 as isize);
            tracer.read_register(s_6_24 as isize, value);
            value
        };
        // C s_6_26: const #3s : i
        let s_6_26: i128 = 3;
        // D s_6_27: cast zx s_6_25 -> bv
        let s_6_27: Bits = Bits::new(s_6_25 as u128, 4u16);
        // C s_6_28: const #1u : u64
        let s_6_28: u64 = 1;
        // D s_6_29: bit-extract s_6_27 s_6_26 s_6_28
        let s_6_29: Bits = (Bits::new(
            ((s_6_27) >> (s_6_26)).value(),
            u16::try_from(s_6_28).unwrap(),
        ));
        // D s_6_30: cast reint s_6_29 -> u8
        let s_6_30: bool = ((s_6_29.value()) != 0);
        // C s_6_31: const #0s : i
        let s_6_31: i128 = 0;
        // C s_6_32: const #0u : u64
        let s_6_32: u64 = 0;
        // D s_6_33: cast zx s_6_30 -> u64
        let s_6_33: u64 = (s_6_30 as u64);
        // C s_6_34: const #1u : u64
        let s_6_34: u64 = 1;
        // D s_6_35: and s_6_33 s_6_34
        let s_6_35: u64 = ((s_6_33) & (s_6_34));
        // D s_6_36: cmp-eq s_6_35 s_6_34
        let s_6_36: bool = ((s_6_35) == (s_6_34));
        // D s_6_37: lsl s_6_33 s_6_31
        let s_6_37: u64 = s_6_33 << s_6_31;
        // D s_6_38: or s_6_32 s_6_37
        let s_6_38: u64 = ((s_6_32) | (s_6_37));
        // D s_6_39: cmpl s_6_37
        let s_6_39: u64 = !s_6_37;
        // D s_6_40: and s_6_32 s_6_39
        let s_6_40: u64 = ((s_6_32) & (s_6_39));
        // D s_6_41: select s_6_36 s_6_38 s_6_40
        let s_6_41: u64 = if s_6_36 { s_6_38 } else { s_6_40 };
        // D s_6_42: cast trunc s_6_41 -> u8
        let s_6_42: bool = ((s_6_41) != 0);
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 1u16);
        // C s_6_44: const #1u : u8
        let s_6_44: bool = true;
        // C s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 1u16);
        // D s_6_46: cmp-eq s_6_43 s_6_45
        let s_6_46: bool = ((s_6_43) == (s_6_45));
        // N s_6_47: branch s_6_46 b9 b7
        if s_6_46 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var m:i64
        let s_7_0: i64 = fn_state.m;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call R_read(s_7_1)
        let s_7_2: u32 = R_read(state, tracer, s_7_1);
        // C s_7_3: const #24s : i
        let s_7_3: i128 = 24;
        // D s_7_4: cast zx s_7_2 -> bv
        let s_7_4: Bits = Bits::new(s_7_2 as u128, 32u16);
        // C s_7_5: const #1s : i64
        let s_7_5: i64 = 1;
        // C s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // C s_7_7: const #7s : i
        let s_7_7: i128 = 7;
        // C s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: bit-extract s_7_4 s_7_3 s_7_8
        let s_7_9: Bits = (Bits::new(
            ((s_7_4) >> (s_7_3)).value(),
            u16::try_from(s_7_8).unwrap(),
        ));
        // D s_7_10: cast reint s_7_9 -> u8
        let s_7_10: u8 = (s_7_9.value() as u8);
        // D s_7_11: write-var ga#346499 <= s_7_10
        fn_state.ga_346499 = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #24s : i
        let s_8_0: i128 = 24;
        // D s_8_1: read-var ga#346498:u32
        let s_8_1: u32 = fn_state.ga_346498;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_3: read-var ga#346499:u8
        let s_8_3: u8 = fn_state.ga_346499;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 8u16);
        // C s_8_5: const #7s : i
        let s_8_5: i128 = 7;
        // C s_8_6: const #1u : u64
        let s_8_6: u64 = 1;
        // C s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 64u16);
        // C s_8_8: lsl s_8_7 s_8_5
        let s_8_8: Bits = s_8_7 << s_8_5;
        // C s_8_9: sub s_8_8 s_8_7
        let s_8_9: Bits = ((s_8_8) - (s_8_7));
        // D s_8_10: and s_8_4 s_8_9
        let s_8_10: Bits = ((s_8_4) & (s_8_9));
        // D s_8_11: lsl s_8_10 s_8_0
        let s_8_11: Bits = s_8_10 << s_8_0;
        // C s_8_12: lsl s_8_9 s_8_0
        let s_8_12: Bits = s_8_9 << s_8_0;
        // C s_8_13: cmpl s_8_12
        let s_8_13: Bits = !s_8_12;
        // D s_8_14: and s_8_2 s_8_13
        let s_8_14: Bits = ((s_8_2) & (s_8_13));
        // D s_8_15: or s_8_14 s_8_11
        let s_8_15: Bits = ((s_8_14) | (s_8_11));
        // D s_8_16: cast reint s_8_15 -> u32
        let s_8_16: u32 = (s_8_15.value() as u32);
        // D s_8_17: read-var d:i64
        let s_8_17: i64 = fn_state.d;
        // D s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_19: call R_set(s_8_18, s_8_16)
        let s_8_19: () = R_set(state, tracer, s_8_18, s_8_16);
        // N s_8_20: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call R_read(s_9_1)
        let s_9_2: u32 = R_read(state, tracer, s_9_1);
        // C s_9_3: const #24s : i
        let s_9_3: i128 = 24;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 32u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #7s : i
        let s_9_7: i128 = 7;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u8
        let s_9_10: u8 = (s_9_9.value() as u8);
        // D s_9_11: write-var ga#346499 <= s_9_10
        fn_state.ga_346499 = s_9_10;
        // N s_9_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call R_read(s_10_1)
        let s_10_2: u32 = R_read(state, tracer, s_10_1);
        // C s_10_3: const #16s : i
        let s_10_3: i128 = 16;
        // D s_10_4: cast zx s_10_2 -> bv
        let s_10_4: Bits = Bits::new(s_10_2 as u128, 32u16);
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // C s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_7: const #7s : i
        let s_10_7: i128 = 7;
        // C s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: bit-extract s_10_4 s_10_3 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_4) >> (s_10_3)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u8
        let s_10_10: u8 = (s_10_9.value() as u8);
        // D s_10_11: write-var ga#346490 <= s_10_10
        fn_state.ga_346490 = s_10_10;
        // N s_10_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var n:i64
        let s_11_0: i64 = fn_state.n;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call R_read(s_11_1)
        let s_11_2: u32 = R_read(state, tracer, s_11_1);
        // C s_11_3: const #8s : i
        let s_11_3: i128 = 8;
        // D s_11_4: cast zx s_11_2 -> bv
        let s_11_4: Bits = Bits::new(s_11_2 as u128, 32u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #7s : i
        let s_11_7: i128 = 7;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_3 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_3)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u8
        let s_11_10: u8 = (s_11_9.value() as u8);
        // D s_11_11: write-var ga#346481 <= s_11_10
        fn_state.ga_346481 = s_11_10;
        // N s_11_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call R_read(s_12_1)
        let s_12_2: u32 = R_read(state, tracer, s_12_1);
        // C s_12_3: const #0s : i
        let s_12_3: i128 = 0;
        // D s_12_4: cast zx s_12_2 -> bv
        let s_12_4: Bits = Bits::new(s_12_2 as u128, 32u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #7s : i
        let s_12_7: i128 = 7;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_3 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_3)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: u8 = (s_12_9.value() as u8);
        // D s_12_11: write-var ga#346472 <= s_12_10
        fn_state.ga_346472 = s_12_10;
        // N s_12_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
