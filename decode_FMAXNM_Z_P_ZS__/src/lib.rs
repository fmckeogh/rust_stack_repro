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
use HaveSVE::*;
use CurrentVL_read::*;
use u__id::*;
use FPOne::*;
use execute_FMAXNM_Z_P_ZS__::*;
use Zeros::*;
use HaveSME::*;
use common::*;
pub fn decode_FMAXNM_Z_P_ZS__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Pg: u8,
    i1: bool,
    Zdn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        dn: i64,
        esize: i64,
        gs_174760: bool,
        gs_174771: bool,
        VL: i64,
        gs_174769: bool,
        imm: Bits,
        g: i64,
        size: u8,
        Pg: u8,
        i1: bool,
        Zdn: u8,
    }
    let fn_state = FunctionState {
        size,
        Pg,
        i1,
        Zdn,
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
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b26 b1
        if s_0_5 {
            return block_26(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#174760 <= s_1_0
        fn_state.gs_174760 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#174760:u8
        let s_2_0: bool = fn_state.gs_174760;
        // N s_2_1: branch s_2_0 b25 b3
        if s_2_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b24 b4
        if s_3_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #8s : i64
        let s_4_4: i64 = 8;
        // D s_4_5: lsl s_4_4 s_4_3
        let s_4_5: i64 = s_4_4 << s_4_3;
        // D s_4_6: write-var esize <= s_4_5
        fn_state.esize = s_4_5;
        // D s_4_7: read-var Pg:u8
        let s_4_7: u8 = fn_state.Pg;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 3u16);
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var g <= s_4_10
        fn_state.g = s_4_10;
        // D s_4_12: read-var Zdn:u8
        let s_4_12: u8 = fn_state.Zdn;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var dn <= s_4_15
        fn_state.dn = s_4_15;
        // D s_4_17: read-var i1:u8
        let s_4_17: bool = fn_state.i1;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #0u : u8
        let s_4_19: bool = false;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b23 b5
        if s_4_21 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #16s : i
        let s_5_4: i128 = 16;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-eq s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) == (s_5_4));
        // N s_5_7: branch s_5_6 b22 b6
        if s_5_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #32s : i
        let s_6_4: i128 = 32;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#174769 <= s_6_6
        fn_state.gs_174769 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#174769:u8
        let s_7_0: bool = fn_state.gs_174769;
        // N s_7_1: branch s_7_0 b21 b8
        if s_7_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #64s : i
        let s_8_4: i128 = 64;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // D s_8_7: write-var gs#174771 <= s_8_6
        fn_state.gs_174771 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#174771:u8
        let s_9_0: bool = fn_state.gs_174771;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var esize:i64
        let s_9_2: i64 = fn_state.esize;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // C s_9_5: const #0u : u8
        let s_9_5: bool = false;
        // D s_9_6: call FPOne(s_9_5, s_9_4)
        let s_9_6: Bits = FPOne(state, tracer, s_9_5, s_9_4);
        // D s_9_7: write-var imm <= s_9_6
        fn_state.imm = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #128s : i
        let s_10_1: i128 = 128;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var dn:i64
        let s_11_4: i64 = fn_state.dn;
        // D s_11_5: read-var g:i64
        let s_11_5: i64 = fn_state.g;
        // D s_11_6: read-var imm:bv
        let s_11_6: Bits = fn_state.imm;
        // D s_11_7: call execute_FMAXNM_Z_P_ZS__(s_11_0, s_11_4, s_11_3, s_11_5, s_11_6)
        let s_11_7: () = execute_FMAXNM_Z_P_ZS__(
            state,
            tracer,
            s_11_0,
            s_11_4,
            s_11_3,
            s_11_5,
            s_11_6,
        );
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #256s : i
        let s_12_1: i128 = 256;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #256s : i64
        let s_13_0: i64 = 256;
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var dn:i64
        let s_13_4: i64 = fn_state.dn;
        // D s_13_5: read-var g:i64
        let s_13_5: i64 = fn_state.g;
        // D s_13_6: read-var imm:bv
        let s_13_6: Bits = fn_state.imm;
        // D s_13_7: call execute_FMAXNM_Z_P_ZS__(s_13_0, s_13_4, s_13_3, s_13_5, s_13_6)
        let s_13_7: () = execute_FMAXNM_Z_P_ZS__(
            state,
            tracer,
            s_13_0,
            s_13_4,
            s_13_3,
            s_13_5,
            s_13_6,
        );
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VL:i64
        let s_14_0: i64 = fn_state.VL;
        // C s_14_1: const #512s : i
        let s_14_1: i128 = 512;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #512s : i64
        let s_15_0: i64 = 512;
        // D s_15_1: read-var esize:i64
        let s_15_1: i64 = fn_state.esize;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var dn:i64
        let s_15_4: i64 = fn_state.dn;
        // D s_15_5: read-var g:i64
        let s_15_5: i64 = fn_state.g;
        // D s_15_6: read-var imm:bv
        let s_15_6: Bits = fn_state.imm;
        // D s_15_7: call execute_FMAXNM_Z_P_ZS__(s_15_0, s_15_4, s_15_3, s_15_5, s_15_6)
        let s_15_7: () = execute_FMAXNM_Z_P_ZS__(
            state,
            tracer,
            s_15_0,
            s_15_4,
            s_15_3,
            s_15_5,
            s_15_6,
        );
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VL:i64
        let s_16_0: i64 = fn_state.VL;
        // C s_16_1: const #1024s : i
        let s_16_1: i128 = 1024;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1024s : i64
        let s_17_0: i64 = 1024;
        // D s_17_1: read-var esize:i64
        let s_17_1: i64 = fn_state.esize;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // D s_17_4: read-var dn:i64
        let s_17_4: i64 = fn_state.dn;
        // D s_17_5: read-var g:i64
        let s_17_5: i64 = fn_state.g;
        // D s_17_6: read-var imm:bv
        let s_17_6: Bits = fn_state.imm;
        // D s_17_7: call execute_FMAXNM_Z_P_ZS__(s_17_0, s_17_4, s_17_3, s_17_5, s_17_6)
        let s_17_7: () = execute_FMAXNM_Z_P_ZS__(
            state,
            tracer,
            s_17_0,
            s_17_4,
            s_17_3,
            s_17_5,
            s_17_6,
        );
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var VL:i64
        let s_18_0: i64 = fn_state.VL;
        // C s_18_1: const #2048s : i
        let s_18_1: i128 = 2048;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
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
        // C s_19_0: const #2048s : i64
        let s_19_0: i64 = 2048;
        // D s_19_1: read-var esize:i64
        let s_19_1: i64 = fn_state.esize;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // D s_19_4: read-var dn:i64
        let s_19_4: i64 = fn_state.dn;
        // D s_19_5: read-var g:i64
        let s_19_5: i64 = fn_state.g;
        // D s_19_6: read-var imm:bv
        let s_19_6: Bits = fn_state.imm;
        // D s_19_7: call execute_FMAXNM_Z_P_ZS__(s_19_0, s_19_4, s_19_3, s_19_5, s_19_6)
        let s_19_7: () = execute_FMAXNM_Z_P_ZS__(
            state,
            tracer,
            s_19_0,
            s_19_4,
            s_19_3,
            s_19_5,
            s_19_6,
        );
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#174771 <= s_21_0
        fn_state.gs_174771 = s_21_0;
        // N s_21_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#174769 <= s_22_0
        fn_state.gs_174769 = s_22_0;
        // N s_22_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esize:i64
        let s_23_0: i64 = fn_state.esize;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call Zeros(s_23_1)
        let s_23_2: Bits = Zeros(state, tracer, s_23_1);
        // D s_23_3: write-var imm <= s_23_2
        fn_state.imm = s_23_2;
        // N s_23_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call HaveSME(s_26_0)
        let s_26_1: bool = HaveSME(state, tracer, s_26_0);
        // S s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // D s_26_3: write-var gs#174760 <= s_26_2
        fn_state.gs_174760 = s_26_2;
        // N s_26_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
