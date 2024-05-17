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
use execute_ADD_Z_ZI__::*;
use CurrentVL_read::*;
use HaveSME::*;
use common::*;
pub fn decode_ADD_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    sh: bool,
    imm8: u8,
    Zdn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        dn: i64,
        VL: i64,
        esize: i64,
        gs_185348: bool,
        immshadow_2472: i128,
        imm: i128,
        size: u8,
        sh: bool,
        imm8: u8,
        Zdn: u8,
    }
    let fn_state = FunctionState {
        size,
        sh,
        imm8,
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
        // N s_0_6: branch s_0_5 b20 b1
        if s_0_5 {
            return block_20(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#185348 <= s_1_0
        fn_state.gs_185348 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#185348:u8
        let s_2_0: bool = fn_state.gs_185348;
        // N s_2_1: branch s_2_0 b19 b3
        if s_2_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_3_2: read-var sh:u8
        let s_3_2: bool = fn_state.sh;
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
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 3u16);
        // C s_3_14: const #1u : u8
        let s_3_14: u8 = 1;
        // C s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 3u16);
        // D s_3_16: cmp-eq s_3_13 s_3_15
        let s_3_16: bool = ((s_3_13) == (s_3_15));
        // N s_3_17: branch s_3_16 b18 b4
        if s_3_16 {
            return block_18(state, tracer, fn_state);
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
        // D s_4_7: read-var Zdn:u8
        let s_4_7: u8 = fn_state.Zdn;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 5u16);
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var dn <= s_4_10
        fn_state.dn = s_4_10;
        // D s_4_12: read-var imm8:u8
        let s_4_12: u8 = fn_state.imm8;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 8u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: write-var imm <= s_4_14
        fn_state.imm = s_4_14;
        // D s_4_16: read-var sh:u8
        let s_4_16: bool = fn_state.sh;
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 1u16);
        // C s_4_18: const #1u : u8
        let s_4_18: bool = true;
        // C s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 1u16);
        // D s_4_20: cmp-eq s_4_17 s_4_19
        let s_4_20: bool = ((s_4_17) == (s_4_19));
        // N s_4_21: branch s_4_20 b17 b5
        if s_4_20 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var imm:i
        let s_6_0: i128 = fn_state.imm;
        // D s_6_1: write-var immshadow#2472 <= s_6_0
        fn_state.immshadow_2472 = s_6_0;
        // D s_6_2: read-var VL:i64
        let s_6_2: i64 = fn_state.VL;
        // C s_6_3: const #128s : i
        let s_6_3: i128 = 128;
        // D s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_5: cmp-eq s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) == (s_6_3));
        // D s_6_6: not s_6_5
        let s_6_6: bool = !s_6_5;
        // N s_6_7: branch s_6_6 b8 b7
        if s_6_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var dn:i64
        let s_7_4: i64 = fn_state.dn;
        // D s_7_5: read-var immshadow#2472:i
        let s_7_5: i128 = fn_state.immshadow_2472;
        // D s_7_6: call execute_ADD_Z_ZI__(s_7_0, s_7_4, s_7_3, s_7_5)
        let s_7_6: () = execute_ADD_Z_ZI__(state, tracer, s_7_0, s_7_4, s_7_3, s_7_5);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #256s : i
        let s_8_1: i128 = 256;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #256s : i64
        let s_9_0: i64 = 256;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var dn:i64
        let s_9_4: i64 = fn_state.dn;
        // D s_9_5: read-var immshadow#2472:i
        let s_9_5: i128 = fn_state.immshadow_2472;
        // D s_9_6: call execute_ADD_Z_ZI__(s_9_0, s_9_4, s_9_3, s_9_5)
        let s_9_6: () = execute_ADD_Z_ZI__(state, tracer, s_9_0, s_9_4, s_9_3, s_9_5);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #512s : i
        let s_10_1: i128 = 512;
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
        // C s_11_0: const #512s : i64
        let s_11_0: i64 = 512;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var dn:i64
        let s_11_4: i64 = fn_state.dn;
        // D s_11_5: read-var immshadow#2472:i
        let s_11_5: i128 = fn_state.immshadow_2472;
        // D s_11_6: call execute_ADD_Z_ZI__(s_11_0, s_11_4, s_11_3, s_11_5)
        let s_11_6: () = execute_ADD_Z_ZI__(
            state,
            tracer,
            s_11_0,
            s_11_4,
            s_11_3,
            s_11_5,
        );
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #1024s : i
        let s_12_1: i128 = 1024;
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
        // C s_13_0: const #1024s : i64
        let s_13_0: i64 = 1024;
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var dn:i64
        let s_13_4: i64 = fn_state.dn;
        // D s_13_5: read-var immshadow#2472:i
        let s_13_5: i128 = fn_state.immshadow_2472;
        // D s_13_6: call execute_ADD_Z_ZI__(s_13_0, s_13_4, s_13_3, s_13_5)
        let s_13_6: () = execute_ADD_Z_ZI__(
            state,
            tracer,
            s_13_0,
            s_13_4,
            s_13_3,
            s_13_5,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VL:i64
        let s_14_0: i64 = fn_state.VL;
        // C s_14_1: const #2048s : i
        let s_14_1: i128 = 2048;
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
        // C s_15_0: const #2048s : i64
        let s_15_0: i64 = 2048;
        // D s_15_1: read-var esize:i64
        let s_15_1: i64 = fn_state.esize;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var dn:i64
        let s_15_4: i64 = fn_state.dn;
        // D s_15_5: read-var immshadow#2472:i
        let s_15_5: i128 = fn_state.immshadow_2472;
        // D s_15_6: call execute_ADD_Z_ZI__(s_15_0, s_15_4, s_15_3, s_15_5)
        let s_15_6: () = execute_ADD_Z_ZI__(
            state,
            tracer,
            s_15_0,
            s_15_4,
            s_15_3,
            s_15_5,
        );
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #8s : i
        let s_17_0: i128 = 8;
        // D s_17_1: read-var imm:i
        let s_17_1: i128 = fn_state.imm;
        // D s_17_2: lsl s_17_1 s_17_0
        let s_17_2: i128 = s_17_1 << s_17_0;
        // D s_17_3: write-var imm <= s_17_2
        fn_state.imm = s_17_2;
        // N s_17_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call HaveSME(s_20_0)
        let s_20_1: bool = HaveSME(state, tracer, s_20_0);
        // S s_20_2: not s_20_1
        let s_20_2: bool = !s_20_1;
        // D s_20_3: write-var gs#185348 <= s_20_2
        fn_state.gs_185348 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
