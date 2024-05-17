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
use CurrentVL_read::*;
use HaveSVE2::*;
use execute_EXT_Z_ZI_Con::*;
use HaveSME::*;
use common::*;
pub fn decode_EXT_Z_ZI_Con<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm8h: u8,
    imm8l: u8,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        dst: i64,
        s2: i64,
        VL: i64,
        s1: i64,
        gs_196512: bool,
        position: i64,
        imm8h: u8,
        imm8l: u8,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        imm8h,
        imm8l,
        Zn,
        Zd,
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
        // S s_0_4: call HaveSVE2(s_0_3)
        let s_0_4: bool = HaveSVE2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#196512 <= s_1_0
        fn_state.gs_196512 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#196512:u8
        let s_2_0: bool = fn_state.gs_196512;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Zd:u8
        let s_3_0: u8 = fn_state.Zd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var dst <= s_3_3
        fn_state.dst = s_3_3;
        // D s_3_5: read-var Zn:u8
        let s_3_5: u8 = fn_state.Zn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var s1 <= s_3_8
        fn_state.s1 = s_3_8;
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: read-var s1:i64
        let s_3_11: i64 = fn_state.s1;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_12 s_3_10
        let s_3_13: i128 = (s_3_12 + s_3_10);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // C s_3_15: const #32s : i
        let s_3_15: i128 = 32;
        // D s_3_16: cast zx s_3_14 -> i
        let s_3_16: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_17: mod s_3_16 s_3_15
        let s_3_17: i128 = ((s_3_16) % (s_3_15));
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var s2 <= s_3_18
        fn_state.s2 = s_3_18;
        // D s_3_20: read-var imm8h:u8
        let s_3_20: u8 = fn_state.imm8h;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 5u16);
        // D s_3_22: read-var imm8l:u8
        let s_3_22: u8 = fn_state.imm8l;
        // D s_3_23: cast zx s_3_22 -> bv
        let s_3_23: Bits = Bits::new(s_3_22 as u128, 3u16);
        // D s_3_24: cast reint s_3_21 -> u128
        let s_3_24: u128 = (s_3_21.value() as u128);
        // D s_3_25: size-of s_3_21
        let s_3_25: u16 = s_3_21.length();
        // D s_3_26: cast reint s_3_23 -> u128
        let s_3_26: u128 = (s_3_23.value() as u128);
        // D s_3_27: size-of s_3_23
        let s_3_27: u16 = s_3_23.length();
        // D s_3_28: lsl s_3_24 s_3_27
        let s_3_28: u128 = s_3_24 << s_3_27;
        // D s_3_29: or s_3_28 s_3_26
        let s_3_29: u128 = ((s_3_28) | (s_3_26));
        // D s_3_30: add s_3_25 s_3_27
        let s_3_30: u16 = (s_3_25 + s_3_27);
        // D s_3_31: create-bits s_3_29 s_3_30
        let s_3_31: Bits = Bits::new(s_3_29, s_3_30);
        // D s_3_32: cast reint s_3_31 -> u8
        let s_3_32: u8 = (s_3_31.value() as u8);
        // D s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 8u16);
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (s_3_33.value() as i128);
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // D s_3_36: write-var position <= s_3_35
        fn_state.position = s_3_35;
        // D s_3_37: read-var VL:i64
        let s_3_37: i64 = fn_state.VL;
        // C s_3_38: const #128s : i
        let s_3_38: i128 = 128;
        // D s_3_39: cast zx s_3_37 -> i
        let s_3_39: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_40: cmp-eq s_3_39 s_3_38
        let s_3_40: bool = ((s_3_39) == (s_3_38));
        // D s_3_41: not s_3_40
        let s_3_41: bool = !s_3_40;
        // N s_3_42: branch s_3_41 b5 b4
        if s_3_41 {
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
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: read-var s2:i64
        let s_4_1: i64 = fn_state.s2;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var dst:i64
        let s_4_3: i64 = fn_state.dst;
        // C s_4_4: const #8s : i64
        let s_4_4: i64 = 8;
        // D s_4_5: read-var position:i64
        let s_4_5: i64 = fn_state.position;
        // D s_4_6: read-var s1:i64
        let s_4_6: i64 = fn_state.s1;
        // D s_4_7: call execute_EXT_Z_ZI_Con(s_4_0, s_4_3, s_4_4, s_4_5, s_4_6, s_4_2)
        let s_4_7: () = execute_EXT_Z_ZI_Con(
            state,
            tracer,
            s_4_0,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_2,
        );
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #256s : i
        let s_5_1: i128 = 256;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // C s_6_0: const #256s : i64
        let s_6_0: i64 = 256;
        // D s_6_1: read-var s2:i64
        let s_6_1: i64 = fn_state.s2;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: read-var dst:i64
        let s_6_3: i64 = fn_state.dst;
        // C s_6_4: const #8s : i64
        let s_6_4: i64 = 8;
        // D s_6_5: read-var position:i64
        let s_6_5: i64 = fn_state.position;
        // D s_6_6: read-var s1:i64
        let s_6_6: i64 = fn_state.s1;
        // D s_6_7: call execute_EXT_Z_ZI_Con(s_6_0, s_6_3, s_6_4, s_6_5, s_6_6, s_6_2)
        let s_6_7: () = execute_EXT_Z_ZI_Con(
            state,
            tracer,
            s_6_0,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_2,
        );
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #512s : i
        let s_7_1: i128 = 512;
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
        // C s_8_0: const #512s : i64
        let s_8_0: i64 = 512;
        // D s_8_1: read-var s2:i64
        let s_8_1: i64 = fn_state.s2;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: read-var dst:i64
        let s_8_3: i64 = fn_state.dst;
        // C s_8_4: const #8s : i64
        let s_8_4: i64 = 8;
        // D s_8_5: read-var position:i64
        let s_8_5: i64 = fn_state.position;
        // D s_8_6: read-var s1:i64
        let s_8_6: i64 = fn_state.s1;
        // D s_8_7: call execute_EXT_Z_ZI_Con(s_8_0, s_8_3, s_8_4, s_8_5, s_8_6, s_8_2)
        let s_8_7: () = execute_EXT_Z_ZI_Con(
            state,
            tracer,
            s_8_0,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_2,
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
        // C s_9_1: const #1024s : i
        let s_9_1: i128 = 1024;
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
        // C s_10_0: const #1024s : i64
        let s_10_0: i64 = 1024;
        // D s_10_1: read-var s2:i64
        let s_10_1: i64 = fn_state.s2;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: read-var dst:i64
        let s_10_3: i64 = fn_state.dst;
        // C s_10_4: const #8s : i64
        let s_10_4: i64 = 8;
        // D s_10_5: read-var position:i64
        let s_10_5: i64 = fn_state.position;
        // D s_10_6: read-var s1:i64
        let s_10_6: i64 = fn_state.s1;
        // D s_10_7: call execute_EXT_Z_ZI_Con(s_10_0, s_10_3, s_10_4, s_10_5, s_10_6, s_10_2)
        let s_10_7: () = execute_EXT_Z_ZI_Con(
            state,
            tracer,
            s_10_0,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_2,
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
        // C s_11_1: const #2048s : i
        let s_11_1: i128 = 2048;
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
        // C s_12_0: const #2048s : i64
        let s_12_0: i64 = 2048;
        // D s_12_1: read-var s2:i64
        let s_12_1: i64 = fn_state.s2;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: read-var dst:i64
        let s_12_3: i64 = fn_state.dst;
        // C s_12_4: const #8s : i64
        let s_12_4: i64 = 8;
        // D s_12_5: read-var position:i64
        let s_12_5: i64 = fn_state.position;
        // D s_12_6: read-var s1:i64
        let s_12_6: i64 = fn_state.s1;
        // D s_12_7: call execute_EXT_Z_ZI_Con(s_12_0, s_12_3, s_12_4, s_12_5, s_12_6, s_12_2)
        let s_12_7: () = execute_EXT_Z_ZI_Con(
            state,
            tracer,
            s_12_0,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_2,
        );
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveSME(s_15_0)
        let s_15_1: bool = HaveSME(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#196512 <= s_15_2
        fn_state.gs_196512 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
