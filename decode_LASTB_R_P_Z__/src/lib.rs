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
use execute_LASTB_R_P_Z__::*;
use HaveSME::*;
use common::*;
pub fn decode_LASTB_R_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    B: bool,
    Pg: u8,
    Zn: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        esize: i64,
        n: i64,
        gs_196840: bool,
        d: i64,
        ga_284712: i64,
        rsize: i64,
        g: i64,
        size: u8,
        B: bool,
        Pg: u8,
        Zn: u8,
        Rd: u8,
    }
    let fn_state = FunctionState {
        size,
        B,
        Pg,
        Zn,
        Rd,
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
        // N s_0_6: branch s_0_5 b18 b1
        if s_0_5 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#196840 <= s_1_0
        fn_state.gs_196840 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#196840:u8
        let s_2_0: bool = fn_state.gs_196840;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #8s : i64
        let s_3_4: i64 = 8;
        // D s_3_5: lsl s_3_4 s_3_3
        let s_3_5: i64 = s_3_4 << s_3_3;
        // D s_3_6: write-var esize <= s_3_5
        fn_state.esize = s_3_5;
        // C s_3_7: const #64s : i
        let s_3_7: i128 = 64;
        // D s_3_8: read-var esize:i64
        let s_3_8: i64 = fn_state.esize;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cmp-lt s_3_9 s_3_7
        let s_3_10: bool = ((s_3_9) < (s_3_7));
        // N s_3_11: branch s_3_10 b16 b4
        if s_3_10 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: write-var ga#284712 <= s_4_0
        fn_state.ga_284712 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#284712:i64
        let s_5_0: i64 = fn_state.ga_284712;
        // D s_5_1: write-var rsize <= s_5_0
        fn_state.rsize = s_5_0;
        // D s_5_2: read-var Pg:u8
        let s_5_2: u8 = fn_state.Pg;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var g <= s_5_5
        fn_state.g = s_5_5;
        // D s_5_7: read-var Zn:u8
        let s_5_7: u8 = fn_state.Zn;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 5u16);
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (s_5_8.value() as i128);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: write-var n <= s_5_10
        fn_state.n = s_5_10;
        // D s_5_12: read-var Rd:u8
        let s_5_12: u8 = fn_state.Rd;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var VL:i64
        let s_5_17: i64 = fn_state.VL;
        // C s_5_18: const #128s : i
        let s_5_18: i128 = 128;
        // D s_5_19: cast zx s_5_17 -> i
        let s_5_19: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_20: cmp-eq s_5_19 s_5_18
        let s_5_20: bool = ((s_5_19) == (s_5_18));
        // D s_5_21: not s_5_20
        let s_5_21: bool = !s_5_20;
        // N s_5_22: branch s_5_21 b7 b6
        if s_5_21 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: read-var esize:i64
        let s_6_1: i64 = fn_state.esize;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: read-var rsize:i64
        let s_6_4: i64 = fn_state.rsize;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // D s_6_7: read-var d:i64
        let s_6_7: i64 = fn_state.d;
        // D s_6_8: read-var g:i64
        let s_6_8: i64 = fn_state.g;
        // C s_6_9: const #1u : u8
        let s_6_9: bool = true;
        // D s_6_10: read-var n:i64
        let s_6_10: i64 = fn_state.n;
        // D s_6_11: call execute_LASTB_R_P_Z__(s_6_0, s_6_7, s_6_3, s_6_8, s_6_9, s_6_10, s_6_6)
        let s_6_11: () = execute_LASTB_R_P_Z__(
            state,
            tracer,
            s_6_0,
            s_6_7,
            s_6_3,
            s_6_8,
            s_6_9,
            s_6_10,
            s_6_6,
        );
        // N s_6_12: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #256s : i
        let s_7_1: i128 = 256;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
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
        // C s_8_0: const #256s : i64
        let s_8_0: i64 = 256;
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var rsize:i64
        let s_8_4: i64 = fn_state.rsize;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // D s_8_7: read-var d:i64
        let s_8_7: i64 = fn_state.d;
        // D s_8_8: read-var g:i64
        let s_8_8: i64 = fn_state.g;
        // C s_8_9: const #1u : u8
        let s_8_9: bool = true;
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // D s_8_11: call execute_LASTB_R_P_Z__(s_8_0, s_8_7, s_8_3, s_8_8, s_8_9, s_8_10, s_8_6)
        let s_8_11: () = execute_LASTB_R_P_Z__(
            state,
            tracer,
            s_8_0,
            s_8_7,
            s_8_3,
            s_8_8,
            s_8_9,
            s_8_10,
            s_8_6,
        );
        // N s_8_12: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #512s : i
        let s_9_1: i128 = 512;
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
        // C s_10_0: const #512s : i64
        let s_10_0: i64 = 512;
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var rsize:i64
        let s_10_4: i64 = fn_state.rsize;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // D s_10_7: read-var d:i64
        let s_10_7: i64 = fn_state.d;
        // D s_10_8: read-var g:i64
        let s_10_8: i64 = fn_state.g;
        // C s_10_9: const #1u : u8
        let s_10_9: bool = true;
        // D s_10_10: read-var n:i64
        let s_10_10: i64 = fn_state.n;
        // D s_10_11: call execute_LASTB_R_P_Z__(s_10_0, s_10_7, s_10_3, s_10_8, s_10_9, s_10_10, s_10_6)
        let s_10_11: () = execute_LASTB_R_P_Z__(
            state,
            tracer,
            s_10_0,
            s_10_7,
            s_10_3,
            s_10_8,
            s_10_9,
            s_10_10,
            s_10_6,
        );
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #1024s : i
        let s_11_1: i128 = 1024;
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
        // C s_12_0: const #1024s : i64
        let s_12_0: i64 = 1024;
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var rsize:i64
        let s_12_4: i64 = fn_state.rsize;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: read-var d:i64
        let s_12_7: i64 = fn_state.d;
        // D s_12_8: read-var g:i64
        let s_12_8: i64 = fn_state.g;
        // C s_12_9: const #1u : u8
        let s_12_9: bool = true;
        // D s_12_10: read-var n:i64
        let s_12_10: i64 = fn_state.n;
        // D s_12_11: call execute_LASTB_R_P_Z__(s_12_0, s_12_7, s_12_3, s_12_8, s_12_9, s_12_10, s_12_6)
        let s_12_11: () = execute_LASTB_R_P_Z__(
            state,
            tracer,
            s_12_0,
            s_12_7,
            s_12_3,
            s_12_8,
            s_12_9,
            s_12_10,
            s_12_6,
        );
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VL:i64
        let s_13_0: i64 = fn_state.VL;
        // C s_13_1: const #2048s : i
        let s_13_1: i128 = 2048;
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
        // C s_14_0: const #2048s : i64
        let s_14_0: i64 = 2048;
        // D s_14_1: read-var esize:i64
        let s_14_1: i64 = fn_state.esize;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var rsize:i64
        let s_14_4: i64 = fn_state.rsize;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: cast reint s_14_5 -> i64
        let s_14_6: i64 = (s_14_5 as i64);
        // D s_14_7: read-var d:i64
        let s_14_7: i64 = fn_state.d;
        // D s_14_8: read-var g:i64
        let s_14_8: i64 = fn_state.g;
        // C s_14_9: const #1u : u8
        let s_14_9: bool = true;
        // D s_14_10: read-var n:i64
        let s_14_10: i64 = fn_state.n;
        // D s_14_11: call execute_LASTB_R_P_Z__(s_14_0, s_14_7, s_14_3, s_14_8, s_14_9, s_14_10, s_14_6)
        let s_14_11: () = execute_LASTB_R_P_Z__(
            state,
            tracer,
            s_14_0,
            s_14_7,
            s_14_3,
            s_14_8,
            s_14_9,
            s_14_10,
            s_14_6,
        );
        // N s_14_12: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #32s : i64
        let s_16_0: i64 = 32;
        // D s_16_1: write-var ga#284712 <= s_16_0
        fn_state.ga_284712 = s_16_0;
        // N s_16_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveSME(s_18_0)
        let s_18_1: bool = HaveSME(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // D s_18_3: write-var gs#196840 <= s_18_2
        fn_state.gs_196840 = s_18_2;
        // N s_18_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
