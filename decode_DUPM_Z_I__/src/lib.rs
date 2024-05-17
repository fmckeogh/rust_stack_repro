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
use execute_DUPM_Z_I__::*;
use DecodeBitMasks::*;
use HaveSME::*;
use common::*;
pub fn decode_DUPM_Z_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm13: u16,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_743694: ProductTypebc91b195b0b2a883,
        VL: i64,
        gs_193086: bool,
        d: i64,
        imm: Bits,
        imm13: u16,
        Zd: u8,
    }
    let fn_state = FunctionState {
        imm13,
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
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b16 b1
        if s_0_5 {
            return block_16(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#193086 <= s_1_0
        fn_state.gs_193086 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#193086:u8
        let s_2_0: bool = fn_state.gs_193086;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_4: write-var d <= s_3_3
        fn_state.d = s_3_3;
        // C s_3_5: const #12s : i
        let s_3_5: i128 = 12;
        // D s_3_6: read-var imm13:u13
        let s_3_6: u16 = fn_state.imm13;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 13u16);
        // C s_3_8: const #1u : u64
        let s_3_8: u64 = 1;
        // D s_3_9: bit-extract s_3_7 s_3_5 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_7) >> (s_3_5)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u8
        let s_3_10: bool = ((s_3_9.value()) != 0);
        // C s_3_11: const #0s : i
        let s_3_11: i128 = 0;
        // C s_3_12: const #0u : u64
        let s_3_12: u64 = 0;
        // D s_3_13: cast zx s_3_10 -> u64
        let s_3_13: u64 = (s_3_10 as u64);
        // C s_3_14: const #1u : u64
        let s_3_14: u64 = 1;
        // D s_3_15: and s_3_13 s_3_14
        let s_3_15: u64 = ((s_3_13) & (s_3_14));
        // D s_3_16: cmp-eq s_3_15 s_3_14
        let s_3_16: bool = ((s_3_15) == (s_3_14));
        // D s_3_17: lsl s_3_13 s_3_11
        let s_3_17: u64 = s_3_13 << s_3_11;
        // D s_3_18: or s_3_12 s_3_17
        let s_3_18: u64 = ((s_3_12) | (s_3_17));
        // D s_3_19: cmpl s_3_17
        let s_3_19: u64 = !s_3_17;
        // D s_3_20: and s_3_12 s_3_19
        let s_3_20: u64 = ((s_3_12) & (s_3_19));
        // D s_3_21: select s_3_16 s_3_18 s_3_20
        let s_3_21: u64 = if s_3_16 { s_3_18 } else { s_3_20 };
        // D s_3_22: cast trunc s_3_21 -> u8
        let s_3_22: bool = ((s_3_21) != 0);
        // C s_3_23: const #0s : i
        let s_3_23: i128 = 0;
        // D s_3_24: read-var imm13:u13
        let s_3_24: u16 = fn_state.imm13;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 13u16);
        // C s_3_26: const #1s : i64
        let s_3_26: i64 = 1;
        // C s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // C s_3_28: const #5s : i
        let s_3_28: i128 = 5;
        // C s_3_29: add s_3_28 s_3_27
        let s_3_29: i128 = (s_3_28 + s_3_27);
        // D s_3_30: bit-extract s_3_25 s_3_23 s_3_29
        let s_3_30: Bits = (Bits::new(
            ((s_3_25) >> (s_3_23)).value(),
            u16::try_from(s_3_29).unwrap(),
        ));
        // D s_3_31: cast reint s_3_30 -> u8
        let s_3_31: u8 = (s_3_30.value() as u8);
        // C s_3_32: const #6s : i
        let s_3_32: i128 = 6;
        // D s_3_33: read-var imm13:u13
        let s_3_33: u16 = fn_state.imm13;
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 13u16);
        // C s_3_35: const #1s : i64
        let s_3_35: i64 = 1;
        // C s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // C s_3_37: const #5s : i
        let s_3_37: i128 = 5;
        // C s_3_38: add s_3_37 s_3_36
        let s_3_38: i128 = (s_3_37 + s_3_36);
        // D s_3_39: bit-extract s_3_34 s_3_32 s_3_38
        let s_3_39: Bits = (Bits::new(
            ((s_3_34) >> (s_3_32)).value(),
            u16::try_from(s_3_38).unwrap(),
        ));
        // D s_3_40: cast reint s_3_39 -> u8
        let s_3_40: u8 = (s_3_39.value() as u8);
        // C s_3_41: const #64s : i64
        let s_3_41: i64 = 64;
        // C s_3_42: cast zx s_3_41 -> i
        let s_3_42: i128 = (i128::try_from(s_3_41).unwrap());
        // C s_3_43: cast reint s_3_42 -> i64
        let s_3_43: i64 = (s_3_42 as i64);
        // C s_3_44: cast zx s_3_43 -> i
        let s_3_44: i128 = (i128::try_from(s_3_43).unwrap());
        // C s_3_45: const #1u : u8
        let s_3_45: bool = true;
        // D s_3_46: call DecodeBitMasks(s_3_22, s_3_31, s_3_40, s_3_45, s_3_44)
        let s_3_46: ProductTypebc91b195b0b2a883 = DecodeBitMasks(
            state,
            tracer,
            s_3_22,
            s_3_31,
            s_3_40,
            s_3_45,
            s_3_44,
        );
        // D s_3_47: write-var gs#743694 <= s_3_46
        fn_state.gs_743694 = s_3_46;
        // N s_3_48: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#743694.0:struct
        let s_4_0: Bits = fn_state.gs_743694._0;
        // D s_4_1: cast reint s_4_0 -> u64
        let s_4_1: u64 = (s_4_0.value() as u64);
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // D s_4_3: write-var imm <= s_4_2
        fn_state.imm = s_4_2;
        // D s_4_4: read-var VL:i64
        let s_4_4: i64 = fn_state.VL;
        // C s_4_5: const #128s : i
        let s_4_5: i128 = 128;
        // D s_4_6: cast zx s_4_4 -> i
        let s_4_6: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_7: cmp-eq s_4_6 s_4_5
        let s_4_7: bool = ((s_4_6) == (s_4_5));
        // D s_4_8: not s_4_7
        let s_4_8: bool = !s_4_7;
        // N s_4_9: branch s_4_8 b6 b5
        if s_4_8 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // C s_5_2: const #64s : i64
        let s_5_2: i64 = 64;
        // D s_5_3: read-var imm:bv
        let s_5_3: Bits = fn_state.imm;
        // D s_5_4: call execute_DUPM_Z_I__(s_5_0, s_5_1, s_5_2, s_5_3)
        let s_5_4: () = execute_DUPM_Z_I__(state, tracer, s_5_0, s_5_1, s_5_2, s_5_3);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #256s : i
        let s_6_1: i128 = 256;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #256s : i64
        let s_7_0: i64 = 256;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // C s_7_2: const #64s : i64
        let s_7_2: i64 = 64;
        // D s_7_3: read-var imm:bv
        let s_7_3: Bits = fn_state.imm;
        // D s_7_4: call execute_DUPM_Z_I__(s_7_0, s_7_1, s_7_2, s_7_3)
        let s_7_4: () = execute_DUPM_Z_I__(state, tracer, s_7_0, s_7_1, s_7_2, s_7_3);
        // N s_7_5: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #512s : i
        let s_8_1: i128 = 512;
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
        // C s_9_0: const #512s : i64
        let s_9_0: i64 = 512;
        // D s_9_1: read-var d:i64
        let s_9_1: i64 = fn_state.d;
        // C s_9_2: const #64s : i64
        let s_9_2: i64 = 64;
        // D s_9_3: read-var imm:bv
        let s_9_3: Bits = fn_state.imm;
        // D s_9_4: call execute_DUPM_Z_I__(s_9_0, s_9_1, s_9_2, s_9_3)
        let s_9_4: () = execute_DUPM_Z_I__(state, tracer, s_9_0, s_9_1, s_9_2, s_9_3);
        // N s_9_5: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #1024s : i
        let s_10_1: i128 = 1024;
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
        // C s_11_0: const #1024s : i64
        let s_11_0: i64 = 1024;
        // D s_11_1: read-var d:i64
        let s_11_1: i64 = fn_state.d;
        // C s_11_2: const #64s : i64
        let s_11_2: i64 = 64;
        // D s_11_3: read-var imm:bv
        let s_11_3: Bits = fn_state.imm;
        // D s_11_4: call execute_DUPM_Z_I__(s_11_0, s_11_1, s_11_2, s_11_3)
        let s_11_4: () = execute_DUPM_Z_I__(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // N s_11_5: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #2048s : i
        let s_12_1: i128 = 2048;
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
        // C s_13_0: const #2048s : i64
        let s_13_0: i64 = 2048;
        // D s_13_1: read-var d:i64
        let s_13_1: i64 = fn_state.d;
        // C s_13_2: const #64s : i64
        let s_13_2: i64 = 64;
        // D s_13_3: read-var imm:bv
        let s_13_3: Bits = fn_state.imm;
        // D s_13_4: call execute_DUPM_Z_I__(s_13_0, s_13_1, s_13_2, s_13_3)
        let s_13_4: () = execute_DUPM_Z_I__(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
        );
        // N s_13_5: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveSME(s_16_0)
        let s_16_1: bool = HaveSME(state, tracer, s_16_0);
        // S s_16_2: not s_16_1
        let s_16_2: bool = !s_16_1;
        // D s_16_3: write-var gs#193086 <= s_16_2
        fn_state.gs_193086 = s_16_2;
        // N s_16_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
