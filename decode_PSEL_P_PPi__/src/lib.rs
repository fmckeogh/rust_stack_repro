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
use HaveSVE2p1::*;
use execute_PSEL_P_PPi__::*;
use CurrentVL_read::*;
use HaveSME::*;
use common::*;
pub fn decode_PSEL_P_PPi__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i1: bool,
    tszh: bool,
    tszl: u8,
    Rv: u8,
    Pn: u8,
    S: bool,
    Pm: u8,
    Pd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esizeshadow_4326: i64,
        esize: i64,
        n: i64,
        imm5: u8,
        v: i64,
        ga_298741: u8,
        VL: i64,
        d: i64,
        immshadow_4325: i64,
        imm: i64,
        gs_219029: bool,
        i1: bool,
        tszh: bool,
        tszl: u8,
        Rv: u8,
        Pn: u8,
        S: bool,
        Pm: u8,
        Pd: u8,
    }
    let fn_state = FunctionState {
        i1,
        tszh,
        tszl,
        Rv,
        Pn,
        S,
        Pm,
        Pd,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME(s_0_3)
        let s_0_4: bool = HaveSME(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b24 b1
        if s_0_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#219029 <= s_1_0
        fn_state.gs_219029 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#219029:u8
        let s_2_0: bool = fn_state.gs_219029;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var i1:u8
        let s_3_0: bool = fn_state.i1;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var tszh:u8
        let s_3_2: bool = fn_state.tszh;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 2u16);
        // D s_3_14: read-var tszl:u8
        let s_3_14: u8 = fn_state.tszl;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 3u16);
        // D s_3_16: cast reint s_3_13 -> u128
        let s_3_16: u128 = (s_3_13.value() as u128);
        // D s_3_17: size-of s_3_13
        let s_3_17: u16 = s_3_13.length();
        // D s_3_18: cast reint s_3_15 -> u128
        let s_3_18: u128 = (s_3_15.value() as u128);
        // D s_3_19: size-of s_3_15
        let s_3_19: u16 = s_3_15.length();
        // D s_3_20: lsl s_3_16 s_3_19
        let s_3_20: u128 = s_3_16 << s_3_19;
        // D s_3_21: or s_3_20 s_3_18
        let s_3_21: u128 = ((s_3_20) | (s_3_18));
        // D s_3_22: add s_3_17 s_3_19
        let s_3_22: u16 = (s_3_17 + s_3_19);
        // D s_3_23: create-bits s_3_21 s_3_22
        let s_3_23: Bits = Bits::new(s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u8
        let s_3_24: u8 = (s_3_23.value() as u8);
        // D s_3_25: write-var imm5 <= s_3_24
        fn_state.imm5 = s_3_24;
        // C s_3_26: const #8s : i64
        let s_3_26: i64 = 8;
        // D s_3_27: write-var esize <= s_3_26
        fn_state.esize = s_3_26;
        // D s_3_28: read-var tszh:u8
        let s_3_28: bool = fn_state.tszh;
        // D s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 1u16);
        // D s_3_30: read-var tszl:u8
        let s_3_30: u8 = fn_state.tszl;
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 3u16);
        // D s_3_32: cast reint s_3_29 -> u128
        let s_3_32: u128 = (s_3_29.value() as u128);
        // D s_3_33: size-of s_3_29
        let s_3_33: u16 = s_3_29.length();
        // D s_3_34: cast reint s_3_31 -> u128
        let s_3_34: u128 = (s_3_31.value() as u128);
        // D s_3_35: size-of s_3_31
        let s_3_35: u16 = s_3_31.length();
        // D s_3_36: lsl s_3_32 s_3_35
        let s_3_36: u128 = s_3_32 << s_3_35;
        // D s_3_37: or s_3_36 s_3_34
        let s_3_37: u128 = ((s_3_36) | (s_3_34));
        // D s_3_38: add s_3_33 s_3_35
        let s_3_38: u16 = (s_3_33 + s_3_35);
        // D s_3_39: create-bits s_3_37 s_3_38
        let s_3_39: Bits = Bits::new(s_3_37, s_3_38);
        // D s_3_40: cast reint s_3_39 -> u8
        let s_3_40: u8 = (s_3_39.value() as u8);
        // D s_3_41: write-var ga#298741 <= s_3_40
        fn_state.ga_298741 = s_3_40;
        // D s_3_42: read-var ga#298741:u8
        let s_3_42: u8 = fn_state.ga_298741;
        // D s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 4u16);
        // C s_3_44: const #0u : u8
        let s_3_44: u8 = 0;
        // C s_3_45: cast zx s_3_44 -> bv
        let s_3_45: Bits = Bits::new(s_3_44 as u128, 4u16);
        // D s_3_46: cmp-eq s_3_43 s_3_45
        let s_3_46: bool = ((s_3_43) == (s_3_45));
        // D s_3_47: not s_3_46
        let s_3_47: bool = !s_3_46;
        // N s_3_48: branch s_3_47 b5 b4
        if s_3_47 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#298741:u8
        let s_5_0: u8 = fn_state.ga_298741;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #8u : u8
        let s_5_2: u8 = 8;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b18 b6
        if s_5_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // D s_6_3: read-var imm5:u8
        let s_6_3: u8 = fn_state.imm5;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 5u16);
        // C s_6_5: const #1u : u64
        let s_6_5: u64 = 1;
        // D s_6_6: bit-extract s_6_4 s_6_2 s_6_5
        let s_6_6: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_5).unwrap(),
        ));
        // D s_6_7: cast reint s_6_6 -> u8
        let s_6_7: bool = ((s_6_6.value()) != 0);
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // C s_6_9: const #0u : u64
        let s_6_9: u64 = 0;
        // D s_6_10: cast zx s_6_7 -> u64
        let s_6_10: u64 = (s_6_7 as u64);
        // C s_6_11: const #1u : u64
        let s_6_11: u64 = 1;
        // D s_6_12: and s_6_10 s_6_11
        let s_6_12: u64 = ((s_6_10) & (s_6_11));
        // D s_6_13: cmp-eq s_6_12 s_6_11
        let s_6_13: bool = ((s_6_12) == (s_6_11));
        // D s_6_14: lsl s_6_10 s_6_8
        let s_6_14: u64 = s_6_10 << s_6_8;
        // D s_6_15: or s_6_9 s_6_14
        let s_6_15: u64 = ((s_6_9) | (s_6_14));
        // D s_6_16: cmpl s_6_14
        let s_6_16: u64 = !s_6_14;
        // D s_6_17: and s_6_9 s_6_16
        let s_6_17: u64 = ((s_6_9) & (s_6_16));
        // D s_6_18: select s_6_13 s_6_15 s_6_17
        let s_6_18: u64 = if s_6_13 { s_6_15 } else { s_6_17 };
        // D s_6_19: cast trunc s_6_18 -> u8
        let s_6_19: bool = ((s_6_18) != 0);
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (s_6_20.value() as i128);
        // D s_6_22: cast reint s_6_21 -> i64
        let s_6_22: i64 = (s_6_21 as i64);
        // D s_6_23: write-var imm <= s_6_22
        fn_state.imm = s_6_22;
        // N s_6_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var imm:i64
        let s_7_0: i64 = fn_state.imm;
        // D s_7_1: write-var immshadow#4325 <= s_7_0
        fn_state.immshadow_4325 = s_7_0;
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: write-var esizeshadow#4326 <= s_7_2
        fn_state.esizeshadow_4326 = s_7_2;
        // D s_7_4: read-var Pn:u8
        let s_7_4: u8 = fn_state.Pn;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 4u16);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (s_7_5.value() as i128);
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: write-var n <= s_7_7
        fn_state.n = s_7_7;
        // D s_7_9: read-var Pm:u8
        let s_7_9: u8 = fn_state.Pm;
        // D s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 4u16);
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (s_7_10.value() as i128);
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: write-var m <= s_7_12
        fn_state.m = s_7_12;
        // D s_7_14: read-var Pd:u8
        let s_7_14: u8 = fn_state.Pd;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 4u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: write-var d <= s_7_17
        fn_state.d = s_7_17;
        // C s_7_19: const #3u : u8
        let s_7_19: u8 = 3;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 3u16);
        // D s_7_21: read-var Rv:u8
        let s_7_21: u8 = fn_state.Rv;
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 2u16);
        // C s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: cast reint s_7_22 -> u128
        let s_7_25: u128 = (s_7_22.value() as u128);
        // D s_7_26: size-of s_7_22
        let s_7_26: u16 = s_7_22.length();
        // D s_7_27: lsl s_7_23 s_7_26
        let s_7_27: u128 = s_7_23 << s_7_26;
        // D s_7_28: or s_7_27 s_7_25
        let s_7_28: u128 = ((s_7_27) | (s_7_25));
        // D s_7_29: add s_7_24 s_7_26
        let s_7_29: u16 = (s_7_24 + s_7_26);
        // D s_7_30: create-bits s_7_28 s_7_29
        let s_7_30: Bits = Bits::new(s_7_28, s_7_29);
        // D s_7_31: cast reint s_7_30 -> u8
        let s_7_31: u8 = (s_7_30.value() as u8);
        // D s_7_32: cast zx s_7_31 -> bv
        let s_7_32: Bits = Bits::new(s_7_31 as u128, 5u16);
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (s_7_32.value() as i128);
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: write-var v <= s_7_34
        fn_state.v = s_7_34;
        // D s_7_36: read-var VL:i64
        let s_7_36: i64 = fn_state.VL;
        // C s_7_37: const #128s : i
        let s_7_37: i128 = 128;
        // D s_7_38: cast zx s_7_36 -> i
        let s_7_38: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_39: cmp-eq s_7_38 s_7_37
        let s_7_39: bool = ((s_7_38) == (s_7_37));
        // D s_7_40: not s_7_39
        let s_7_40: bool = !s_7_39;
        // N s_7_41: branch s_7_40 b9 b8
        if s_7_40 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: read-var d:i64
        let s_8_1: i64 = fn_state.d;
        // D s_8_2: read-var esizeshadow#4326:i64
        let s_8_2: i64 = fn_state.esizeshadow_4326;
        // D s_8_3: read-var immshadow#4325:i64
        let s_8_3: i64 = fn_state.immshadow_4325;
        // D s_8_4: read-var m:i64
        let s_8_4: i64 = fn_state.m;
        // D s_8_5: read-var n:i64
        let s_8_5: i64 = fn_state.n;
        // D s_8_6: read-var v:i64
        let s_8_6: i64 = fn_state.v;
        // D s_8_7: call execute_PSEL_P_PPi__(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4, s_8_5, s_8_6)
        let s_8_7: () = execute_PSEL_P_PPi__(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
        );
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #256s : i
        let s_9_1: i128 = 256;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #256s : i64
        let s_10_0: i64 = 256;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: read-var esizeshadow#4326:i64
        let s_10_2: i64 = fn_state.esizeshadow_4326;
        // D s_10_3: read-var immshadow#4325:i64
        let s_10_3: i64 = fn_state.immshadow_4325;
        // D s_10_4: read-var m:i64
        let s_10_4: i64 = fn_state.m;
        // D s_10_5: read-var n:i64
        let s_10_5: i64 = fn_state.n;
        // D s_10_6: read-var v:i64
        let s_10_6: i64 = fn_state.v;
        // D s_10_7: call execute_PSEL_P_PPi__(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5, s_10_6)
        let s_10_7: () = execute_PSEL_P_PPi__(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
        );
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #512s : i
        let s_11_1: i128 = 512;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #512s : i64
        let s_12_0: i64 = 512;
        // D s_12_1: read-var d:i64
        let s_12_1: i64 = fn_state.d;
        // D s_12_2: read-var esizeshadow#4326:i64
        let s_12_2: i64 = fn_state.esizeshadow_4326;
        // D s_12_3: read-var immshadow#4325:i64
        let s_12_3: i64 = fn_state.immshadow_4325;
        // D s_12_4: read-var m:i64
        let s_12_4: i64 = fn_state.m;
        // D s_12_5: read-var n:i64
        let s_12_5: i64 = fn_state.n;
        // D s_12_6: read-var v:i64
        let s_12_6: i64 = fn_state.v;
        // D s_12_7: call execute_PSEL_P_PPi__(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5, s_12_6)
        let s_12_7: () = execute_PSEL_P_PPi__(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
        );
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VL:i64
        let s_13_0: i64 = fn_state.VL;
        // C s_13_1: const #1024s : i
        let s_13_1: i128 = 1024;
        // D s_13_2: cast zx s_13_0 -> i
        let s_13_2: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_1
        let s_13_3: bool = ((s_13_2) == (s_13_1));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b15 b14
        if s_13_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1024s : i64
        let s_14_0: i64 = 1024;
        // D s_14_1: read-var d:i64
        let s_14_1: i64 = fn_state.d;
        // D s_14_2: read-var esizeshadow#4326:i64
        let s_14_2: i64 = fn_state.esizeshadow_4326;
        // D s_14_3: read-var immshadow#4325:i64
        let s_14_3: i64 = fn_state.immshadow_4325;
        // D s_14_4: read-var m:i64
        let s_14_4: i64 = fn_state.m;
        // D s_14_5: read-var n:i64
        let s_14_5: i64 = fn_state.n;
        // D s_14_6: read-var v:i64
        let s_14_6: i64 = fn_state.v;
        // D s_14_7: call execute_PSEL_P_PPi__(s_14_0, s_14_1, s_14_2, s_14_3, s_14_4, s_14_5, s_14_6)
        let s_14_7: () = execute_PSEL_P_PPi__(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_5,
            s_14_6,
        );
        // N s_14_8: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var VL:i64
        let s_15_0: i64 = fn_state.VL;
        // C s_15_1: const #2048s : i
        let s_15_1: i128 = 2048;
        // D s_15_2: cast zx s_15_0 -> i
        let s_15_2: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_1
        let s_15_3: bool = ((s_15_2) == (s_15_1));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2048s : i64
        let s_16_0: i64 = 2048;
        // D s_16_1: read-var d:i64
        let s_16_1: i64 = fn_state.d;
        // D s_16_2: read-var esizeshadow#4326:i64
        let s_16_2: i64 = fn_state.esizeshadow_4326;
        // D s_16_3: read-var immshadow#4325:i64
        let s_16_3: i64 = fn_state.immshadow_4325;
        // D s_16_4: read-var m:i64
        let s_16_4: i64 = fn_state.m;
        // D s_16_5: read-var n:i64
        let s_16_5: i64 = fn_state.n;
        // D s_16_6: read-var v:i64
        let s_16_6: i64 = fn_state.v;
        // D s_16_7: call execute_PSEL_P_PPi__(s_16_0, s_16_1, s_16_2, s_16_3, s_16_4, s_16_5, s_16_6)
        let s_16_7: () = execute_PSEL_P_PPi__(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
            s_16_5,
            s_16_6,
        );
        // N s_16_8: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var ga#298741:u8
        let s_18_0: u8 = fn_state.ga_298741;
        // C s_18_1: const #0s : i
        let s_18_1: i128 = 0;
        // D s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 4u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #2s : i
        let s_18_5: i128 = 2;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_1 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_1)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 3u16);
        // C s_18_10: const #4u : u8
        let s_18_10: u8 = 4;
        // C s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 3u16);
        // D s_18_12: cmp-eq s_18_9 s_18_11
        let s_18_12: bool = ((s_18_9) == (s_18_11));
        // D s_18_13: not s_18_12
        let s_18_13: bool = !s_18_12;
        // N s_18_14: branch s_18_13 b20 b19
        if s_18_13 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // C s_19_2: const #3s : i
        let s_19_2: i128 = 3;
        // D s_19_3: read-var imm5:u8
        let s_19_3: u8 = fn_state.imm5;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 5u16);
        // C s_19_5: const #1s : i64
        let s_19_5: i64 = 1;
        // C s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // C s_19_7: const #1s : i
        let s_19_7: i128 = 1;
        // C s_19_8: add s_19_7 s_19_6
        let s_19_8: i128 = (s_19_7 + s_19_6);
        // D s_19_9: bit-extract s_19_4 s_19_2 s_19_8
        let s_19_9: Bits = (Bits::new(
            ((s_19_4) >> (s_19_2)).value(),
            u16::try_from(s_19_8).unwrap(),
        ));
        // D s_19_10: cast reint s_19_9 -> u8
        let s_19_10: u8 = (s_19_9.value() as u8);
        // D s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 2u16);
        // D s_19_12: cast zx s_19_11 -> i
        let s_19_12: i128 = (s_19_11.value() as i128);
        // D s_19_13: cast reint s_19_12 -> i64
        let s_19_13: i64 = (s_19_12 as i64);
        // D s_19_14: write-var imm <= s_19_13
        fn_state.imm = s_19_13;
        // N s_19_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#298741:u8
        let s_20_0: u8 = fn_state.ga_298741;
        // C s_20_1: const #0s : i
        let s_20_1: i128 = 0;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #1s : i
        let s_20_5: i128 = 1;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: u8 = (s_20_7.value() as u8);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 2u16);
        // C s_20_10: const #2u : u8
        let s_20_10: u8 = 2;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 2u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b22 b21
        if s_20_13 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #16s : i64
        let s_21_0: i64 = 16;
        // D s_21_1: write-var esize <= s_21_0
        fn_state.esize = s_21_0;
        // C s_21_2: const #2s : i
        let s_21_2: i128 = 2;
        // D s_21_3: read-var imm5:u8
        let s_21_3: u8 = fn_state.imm5;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 5u16);
        // C s_21_5: const #1s : i64
        let s_21_5: i64 = 1;
        // C s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // C s_21_7: const #2s : i
        let s_21_7: i128 = 2;
        // C s_21_8: add s_21_7 s_21_6
        let s_21_8: i128 = (s_21_7 + s_21_6);
        // D s_21_9: bit-extract s_21_4 s_21_2 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_4) >> (s_21_2)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: u8 = (s_21_9.value() as u8);
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 3u16);
        // D s_21_12: cast zx s_21_11 -> i
        let s_21_12: i128 = (s_21_11.value() as i128);
        // D s_21_13: cast reint s_21_12 -> i64
        let s_21_13: i64 = (s_21_12 as i64);
        // D s_21_14: write-var imm <= s_21_13
        fn_state.imm = s_21_13;
        // N s_21_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #8s : i64
        let s_22_0: i64 = 8;
        // D s_22_1: write-var esize <= s_22_0
        fn_state.esize = s_22_0;
        // C s_22_2: const #1s : i
        let s_22_2: i128 = 1;
        // D s_22_3: read-var imm5:u8
        let s_22_3: u8 = fn_state.imm5;
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 5u16);
        // C s_22_5: const #1s : i64
        let s_22_5: i64 = 1;
        // C s_22_6: cast zx s_22_5 -> i
        let s_22_6: i128 = (i128::try_from(s_22_5).unwrap());
        // C s_22_7: const #3s : i
        let s_22_7: i128 = 3;
        // C s_22_8: add s_22_7 s_22_6
        let s_22_8: i128 = (s_22_7 + s_22_6);
        // D s_22_9: bit-extract s_22_4 s_22_2 s_22_8
        let s_22_9: Bits = (Bits::new(
            ((s_22_4) >> (s_22_2)).value(),
            u16::try_from(s_22_8).unwrap(),
        ));
        // D s_22_10: cast reint s_22_9 -> u8
        let s_22_10: u8 = (s_22_9.value() as u8);
        // D s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 4u16);
        // D s_22_12: cast zx s_22_11 -> i
        let s_22_12: i128 = (s_22_11.value() as i128);
        // D s_22_13: cast reint s_22_12 -> i64
        let s_22_13: i64 = (s_22_12 as i64);
        // D s_22_14: write-var imm <= s_22_13
        fn_state.imm = s_22_13;
        // N s_22_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveSVE2p1(s_24_0)
        let s_24_1: bool = HaveSVE2p1(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // D s_24_3: write-var gs#219029 <= s_24_2
        fn_state.gs_219029 = s_24_2;
        // N s_24_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
