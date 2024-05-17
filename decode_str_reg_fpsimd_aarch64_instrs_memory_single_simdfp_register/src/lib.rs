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
use execute_aarch64_instrs_memory_single_simdfp_register::*;
use DecodeRegExtend::*;
use u__id::*;
use common::*;
pub fn decode_str_reg_fpsimd_aarch64_instrs_memory_single_simdfp_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    S: bool,
    option_name: u8,
    Rm: u8,
    opc: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        tagchecked: bool,
        t: i64,
        gs_163992: bool,
        shift: i64,
        gs_163980: bool,
        n: i64,
        memop: u32,
        gs_163986: bool,
        gs_163988: bool,
        datasize: i128,
        extend_type: u32,
        ga_262613: i64,
        scale: i64,
        gs_163984: bool,
        gs_163982: bool,
        gs_163990: bool,
        Rt: u8,
        Rn: u8,
        S: bool,
        option_name: u8,
        Rm: u8,
        opc: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        S,
        option_name,
        Rm,
        opc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var opc:u8
        let s_0_1: u8 = fn_state.opc;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: read-var size:u8
        let s_0_19: u8 = fn_state.size;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // D s_0_21: cast reint s_0_18 -> u128
        let s_0_21: u128 = (s_0_18.value() as u128);
        // D s_0_22: size-of s_0_18
        let s_0_22: u16 = s_0_18.length();
        // D s_0_23: cast reint s_0_20 -> u128
        let s_0_23: u128 = (s_0_20.value() as u128);
        // D s_0_24: size-of s_0_20
        let s_0_24: u16 = s_0_20.length();
        // D s_0_25: lsl s_0_21 s_0_24
        let s_0_25: u128 = s_0_21 << s_0_24;
        // D s_0_26: or s_0_25 s_0_23
        let s_0_26: u128 = ((s_0_25) | (s_0_23));
        // D s_0_27: add s_0_22 s_0_24
        let s_0_27: u16 = (s_0_22 + s_0_24);
        // D s_0_28: create-bits s_0_26 s_0_27
        let s_0_28: Bits = Bits::new(s_0_26, s_0_27);
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: u8 = (s_0_28.value() as u8);
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 3u16);
        // D s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (s_0_30.value() as i128);
        // D s_0_32: cast reint s_0_31 -> i64
        let s_0_32: i64 = (s_0_31 as i64);
        // D s_0_33: write-var scale <= s_0_32
        fn_state.scale = s_0_32;
        // C s_0_34: const #4s : i
        let s_0_34: i128 = 4;
        // D s_0_35: read-var scale:i64
        let s_0_35: i64 = fn_state.scale;
        // D s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_37: cmp-gt s_0_36 s_0_34
        let s_0_37: bool = ((s_0_36) > (s_0_34));
        // N s_0_38: branch s_0_37 b31 b1
        if s_0_37 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i
        let s_1_0: i128 = 1;
        // D s_1_1: read-var option_name:u8
        let s_1_1: u8 = fn_state.option_name;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 3u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #0u : u8
        let s_1_19: bool = false;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b30 b2
        if s_1_21 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var option_name:u8
        let s_2_0: u8 = fn_state.option_name;
        // D s_2_1: call DecodeRegExtend(s_2_0)
        let s_2_1: u32 = DecodeRegExtend(state, tracer, s_2_0);
        // D s_2_2: write-var extend_type <= s_2_1
        fn_state.extend_type = s_2_1;
        // D s_2_3: read-var S:u8
        let s_2_3: bool = fn_state.S;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // C s_2_5: const #1u : u8
        let s_2_5: bool = true;
        // C s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_7: cmp-eq s_2_4 s_2_6
        let s_2_7: bool = ((s_2_4) == (s_2_6));
        // N s_2_8: branch s_2_7 b29 b3
        if s_2_7 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var ga#262613 <= s_3_0
        fn_state.ga_262613 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#262613:i64
        let s_4_0: i64 = fn_state.ga_262613;
        // D s_4_1: write-var shift <= s_4_0
        fn_state.shift = s_4_0;
        // D s_4_2: read-var Rn:u8
        let s_4_2: u8 = fn_state.Rn;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: write-var n <= s_4_5
        fn_state.n = s_4_5;
        // D s_4_7: read-var Rt:u8
        let s_4_7: u8 = fn_state.Rt;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 5u16);
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var t <= s_4_10
        fn_state.t = s_4_10;
        // D s_4_12: read-var Rm:u8
        let s_4_12: u8 = fn_state.Rm;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var m <= s_4_15
        fn_state.m = s_4_15;
        // C s_4_17: const #0s : i
        let s_4_17: i128 = 0;
        // D s_4_18: read-var opc:u8
        let s_4_18: u8 = fn_state.opc;
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 2u16);
        // C s_4_20: const #1u : u64
        let s_4_20: u64 = 1;
        // D s_4_21: bit-extract s_4_19 s_4_17 s_4_20
        let s_4_21: Bits = (Bits::new(
            ((s_4_19) >> (s_4_17)).value(),
            u16::try_from(s_4_20).unwrap(),
        ));
        // D s_4_22: cast reint s_4_21 -> u8
        let s_4_22: bool = ((s_4_21.value()) != 0);
        // C s_4_23: const #0s : i
        let s_4_23: i128 = 0;
        // C s_4_24: const #0u : u64
        let s_4_24: u64 = 0;
        // D s_4_25: cast zx s_4_22 -> u64
        let s_4_25: u64 = (s_4_22 as u64);
        // C s_4_26: const #1u : u64
        let s_4_26: u64 = 1;
        // D s_4_27: and s_4_25 s_4_26
        let s_4_27: u64 = ((s_4_25) & (s_4_26));
        // D s_4_28: cmp-eq s_4_27 s_4_26
        let s_4_28: bool = ((s_4_27) == (s_4_26));
        // D s_4_29: lsl s_4_25 s_4_23
        let s_4_29: u64 = s_4_25 << s_4_23;
        // D s_4_30: or s_4_24 s_4_29
        let s_4_30: u64 = ((s_4_24) | (s_4_29));
        // D s_4_31: cmpl s_4_29
        let s_4_31: u64 = !s_4_29;
        // D s_4_32: and s_4_24 s_4_31
        let s_4_32: u64 = ((s_4_24) & (s_4_31));
        // D s_4_33: select s_4_28 s_4_30 s_4_32
        let s_4_33: u64 = if s_4_28 { s_4_30 } else { s_4_32 };
        // D s_4_34: cast trunc s_4_33 -> u8
        let s_4_34: bool = ((s_4_33) != 0);
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 1u16);
        // C s_4_36: const #1u : u8
        let s_4_36: bool = true;
        // C s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 1u16);
        // D s_4_38: cmp-eq s_4_35 s_4_37
        let s_4_38: bool = ((s_4_35) == (s_4_37));
        // N s_4_39: branch s_4_38 b28 b5
        if s_4_38 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: write-var memop <= s_5_0
        fn_state.memop = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i
        let s_6_0: i128 = 8;
        // D s_6_1: read-var scale:i64
        let s_6_1: i64 = fn_state.scale;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: lsl s_6_0 s_6_2
        let s_6_3: i128 = s_6_0 << s_6_2;
        // D s_6_4: write-var datasize <= s_6_3
        fn_state.datasize = s_6_3;
        // D s_6_5: read-var memop:u32
        let s_6_5: u32 = fn_state.memop;
        // C s_6_6: const #2u : u32
        let s_6_6: u32 = 2;
        // D s_6_7: cmp-eq s_6_5 s_6_6
        let s_6_7: bool = ((s_6_5) == (s_6_6));
        // D s_6_8: write-var tagchecked <= s_6_7
        fn_state.tagchecked = s_6_7;
        // D s_6_9: read-var datasize:i
        let s_6_9: i128 = fn_state.datasize;
        // D s_6_10: call __id(s_6_9)
        let s_6_10: i128 = u__id(state, tracer, s_6_9);
        // C s_6_11: const #8s : i
        let s_6_11: i128 = 8;
        // D s_6_12: cmp-eq s_6_10 s_6_11
        let s_6_12: bool = ((s_6_10) == (s_6_11));
        // N s_6_13: branch s_6_12 b27 b7
        if s_6_12 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasize:i
        let s_7_0: i128 = fn_state.datasize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #16s : i
        let s_7_2: i128 = 16;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#163980 <= s_7_3
        fn_state.gs_163980 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#163980:u8
        let s_8_0: bool = fn_state.gs_163980;
        // N s_8_1: branch s_8_0 b26 b9
        if s_8_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasize:i
        let s_9_0: i128 = fn_state.datasize;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #32s : i
        let s_9_2: i128 = 32;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#163982 <= s_9_3
        fn_state.gs_163982 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#163982:u8
        let s_10_0: bool = fn_state.gs_163982;
        // N s_10_1: branch s_10_0 b25 b11
        if s_10_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var datasize:i
        let s_11_0: i128 = fn_state.datasize;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #64s : i
        let s_11_2: i128 = 64;
        // D s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: write-var gs#163984 <= s_11_3
        fn_state.gs_163984 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#163984:u8
        let s_12_0: bool = fn_state.gs_163984;
        // N s_12_1: branch s_12_0 b24 b13
        if s_12_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasize:i
        let s_13_0: i128 = fn_state.datasize;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #128s : i
        let s_13_2: i128 = 128;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: write-var gs#163986 <= s_13_3
        fn_state.gs_163986 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#163986:u8
        let s_14_0: bool = fn_state.gs_163986;
        // N s_14_1: branch s_14_0 b23 b15
        if s_14_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var datasize:i
        let s_15_0: i128 = fn_state.datasize;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #256s : i
        let s_15_2: i128 = 256;
        // D s_15_3: cmp-eq s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) == (s_15_2));
        // D s_15_4: write-var gs#163988 <= s_15_3
        fn_state.gs_163988 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#163988:u8
        let s_16_0: bool = fn_state.gs_163988;
        // N s_16_1: branch s_16_0 b22 b17
        if s_16_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var datasize:i
        let s_17_0: i128 = fn_state.datasize;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // C s_17_2: const #512s : i
        let s_17_2: i128 = 512;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // D s_17_4: write-var gs#163990 <= s_17_3
        fn_state.gs_163990 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#163990:u8
        let s_18_0: bool = fn_state.gs_163990;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var datasize:i
        let s_19_0: i128 = fn_state.datasize;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #1024s : i
        let s_19_2: i128 = 1024;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#163992 <= s_19_3
        fn_state.gs_163992 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#163992:u8
        let s_20_0: bool = fn_state.gs_163992;
        // N s_20_1: assert s_20_0
        let s_20_1: () = assert!(s_20_0);
        // D s_20_2: read-var datasize:i
        let s_20_2: i128 = fn_state.datasize;
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // D s_20_4: read-var extend_type:u32
        let s_20_4: u32 = fn_state.extend_type;
        // D s_20_5: read-var m:i64
        let s_20_5: i64 = fn_state.m;
        // D s_20_6: read-var memop:u32
        let s_20_6: u32 = fn_state.memop;
        // D s_20_7: read-var n:i64
        let s_20_7: i64 = fn_state.n;
        // C s_20_8: const #0u : u8
        let s_20_8: bool = false;
        // C s_20_9: const #0u : u8
        let s_20_9: bool = false;
        // D s_20_10: read-var shift:i64
        let s_20_10: i64 = fn_state.shift;
        // D s_20_11: read-var t:i64
        let s_20_11: i64 = fn_state.t;
        // D s_20_12: read-var tagchecked:u8
        let s_20_12: bool = fn_state.tagchecked;
        // C s_20_13: const #0u : u8
        let s_20_13: bool = false;
        // D s_20_14: call execute_aarch64_instrs_memory_single_simdfp_register(s_20_3, s_20_4, s_20_5, s_20_6, s_20_7, s_20_8, s_20_9, s_20_10, s_20_11, s_20_12, s_20_13)
        let s_20_14: () = execute_aarch64_instrs_memory_single_simdfp_register(
            state,
            tracer,
            s_20_3,
            s_20_4,
            s_20_5,
            s_20_6,
            s_20_7,
            s_20_8,
            s_20_9,
            s_20_10,
            s_20_11,
            s_20_12,
            s_20_13,
        );
        // N s_20_15: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#163992 <= s_21_0
        fn_state.gs_163992 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#163990 <= s_22_0
        fn_state.gs_163990 = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#163988 <= s_23_0
        fn_state.gs_163988 = s_23_0;
        // N s_23_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#163986 <= s_24_0
        fn_state.gs_163986 = s_24_0;
        // N s_24_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#163984 <= s_25_0
        fn_state.gs_163984 = s_25_0;
        // N s_25_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#163982 <= s_26_0
        fn_state.gs_163982 = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#163980 <= s_27_0
        fn_state.gs_163980 = s_27_0;
        // N s_27_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: write-var memop <= s_28_0
        fn_state.memop = s_28_0;
        // N s_28_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var scale:i64
        let s_29_0: i64 = fn_state.scale;
        // D s_29_1: write-var ga#262613 <= s_29_0
        fn_state.ga_262613 = s_29_0;
        // N s_29_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
}
