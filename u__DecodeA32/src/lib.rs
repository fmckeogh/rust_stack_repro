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
use u__DecodeA32_LoadStoreReg::*;
use u__DecodeA32_SysASIMDFP::*;
use u__DecodeA32_DataProMisc::*;
use u__DecodeA32_Unconditional::*;
use u__DecodeA32_BranchBlock::*;
use u__DecodeA32_Media::*;
use u__DecodeA32_LoadStoreImmLit::*;
use common::*;
pub fn u__DecodeA32<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pc: i128,
    opcode: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        v__40: u32,
        v__36: u32,
        gs_428945: bool,
        gs_428951: bool,
        pc: i128,
        opcode: u32,
    }
    let fn_state = FunctionState {
        pc,
        opcode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var opcode:u32
        let s_0_0: u32 = fn_state.opcode;
        // C s_0_1: const #27s : i
        let s_0_1: i128 = 27;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 32u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #4s : i
        let s_0_5: i128 = 4;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_1 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 5u16);
        // C s_0_10: const #30u : u8
        let s_0_10: u8 = 30;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cmp-eq s_0_9 s_0_11
        let s_0_12: bool = ((s_0_9) == (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b2 b1
        if s_0_13 {
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
        // D s_1_0: read-var pc:i
        let s_1_0: i128 = fn_state.pc;
        // D s_1_1: read-var opcode:u32
        let s_1_1: u32 = fn_state.opcode;
        // D s_1_2: call __DecodeA32_Unconditional(s_1_0, s_1_1)
        let s_1_2: () = u__DecodeA32_Unconditional(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var opcode:u32
        let s_2_0: u32 = fn_state.opcode;
        // C s_2_1: const #26s : i
        let s_2_1: i128 = 26;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 32u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #1s : i
        let s_2_5: i128 = 1;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_1 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_1)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // C s_2_10: const #0u : u8
        let s_2_10: u8 = 0;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 2u16);
        // D s_2_12: cmp-eq s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) == (s_2_11));
        // D s_2_13: not s_2_12
        let s_2_13: bool = !s_2_12;
        // N s_2_14: branch s_2_13 b4 b3
        if s_2_13 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var pc:i
        let s_3_0: i128 = fn_state.pc;
        // D s_3_1: read-var opcode:u32
        let s_3_1: u32 = fn_state.opcode;
        // D s_3_2: call __DecodeA32_DataProMisc(s_3_0, s_3_1)
        let s_3_2: () = u__DecodeA32_DataProMisc(state, tracer, s_3_0, s_3_1);
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var opcode:u32
        let s_4_0: u32 = fn_state.opcode;
        // C s_4_1: const #25s : i
        let s_4_1: i128 = 25;
        // D s_4_2: cast zx s_4_0 -> bv
        let s_4_2: Bits = Bits::new(s_4_0 as u128, 32u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #2s : i
        let s_4_5: i128 = 2;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_1 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_1)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 3u16);
        // C s_4_10: const #2u : u8
        let s_4_10: u8 = 2;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 3u16);
        // D s_4_12: cmp-eq s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) == (s_4_11));
        // D s_4_13: not s_4_12
        let s_4_13: bool = !s_4_12;
        // N s_4_14: branch s_4_13 b6 b5
        if s_4_13 {
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
        // D s_5_0: read-var pc:i
        let s_5_0: i128 = fn_state.pc;
        // D s_5_1: read-var opcode:u32
        let s_5_1: u32 = fn_state.opcode;
        // D s_5_2: call __DecodeA32_LoadStoreImmLit(s_5_0, s_5_1)
        let s_5_2: () = u__DecodeA32_LoadStoreImmLit(state, tracer, s_5_0, s_5_1);
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var opcode:u32
        let s_6_0: u32 = fn_state.opcode;
        // D s_6_1: write-var v__36 <= s_6_0
        fn_state.v__36 = s_6_0;
        // C s_6_2: const #25s : i
        let s_6_2: i128 = 25;
        // D s_6_3: read-var v__36:u32
        let s_6_3: u32 = fn_state.v__36;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #2s : i
        let s_6_7: i128 = 2;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 3u16);
        // C s_6_12: const #3u : u8
        let s_6_12: u8 = 3;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 3u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b18 b7
        if s_6_14 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#428945 <= s_7_0
        fn_state.gs_428945 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#428945:u8
        let s_8_0: bool = fn_state.gs_428945;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
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
        // D s_9_0: read-var pc:i
        let s_9_0: i128 = fn_state.pc;
        // D s_9_1: read-var opcode:u32
        let s_9_1: u32 = fn_state.opcode;
        // D s_9_2: call __DecodeA32_LoadStoreReg(s_9_0, s_9_1)
        let s_9_2: () = u__DecodeA32_LoadStoreReg(state, tracer, s_9_0, s_9_1);
        // N s_9_3: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var opcode:u32
        let s_10_0: u32 = fn_state.opcode;
        // D s_10_1: write-var v__40 <= s_10_0
        fn_state.v__40 = s_10_0;
        // C s_10_2: const #25s : i
        let s_10_2: i128 = 25;
        // D s_10_3: read-var v__40:u32
        let s_10_3: u32 = fn_state.v__40;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 32u16);
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // C s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_7: const #2s : i
        let s_10_7: i128 = 2;
        // C s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: bit-extract s_10_4 s_10_2 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_4) >> (s_10_2)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u8
        let s_10_10: u8 = (s_10_9.value() as u8);
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 3u16);
        // C s_10_12: const #3u : u8
        let s_10_12: u8 = 3;
        // C s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 3u16);
        // D s_10_14: cmp-eq s_10_11 s_10_13
        let s_10_14: bool = ((s_10_11) == (s_10_13));
        // N s_10_15: branch s_10_14 b17 b11
        if s_10_14 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#428951 <= s_11_0
        fn_state.gs_428951 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#428951:u8
        let s_12_0: bool = fn_state.gs_428951;
        // D s_12_1: not s_12_0
        let s_12_1: bool = !s_12_0;
        // N s_12_2: branch s_12_1 b14 b13
        if s_12_1 {
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
        // D s_13_0: read-var pc:i
        let s_13_0: i128 = fn_state.pc;
        // D s_13_1: read-var opcode:u32
        let s_13_1: u32 = fn_state.opcode;
        // D s_13_2: call __DecodeA32_Media(s_13_0, s_13_1)
        let s_13_2: () = u__DecodeA32_Media(state, tracer, s_13_0, s_13_1);
        // N s_13_3: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var opcode:u32
        let s_14_0: u32 = fn_state.opcode;
        // C s_14_1: const #26s : i
        let s_14_1: i128 = 26;
        // D s_14_2: cast zx s_14_0 -> bv
        let s_14_2: Bits = Bits::new(s_14_0 as u128, 32u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #1s : i
        let s_14_5: i128 = 1;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_1 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u8
        let s_14_8: u8 = (s_14_7.value() as u8);
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 2u16);
        // C s_14_10: const #2u : u8
        let s_14_10: u8 = 2;
        // C s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 2u16);
        // D s_14_12: cmp-eq s_14_9 s_14_11
        let s_14_12: bool = ((s_14_9) == (s_14_11));
        // D s_14_13: not s_14_12
        let s_14_13: bool = !s_14_12;
        // N s_14_14: branch s_14_13 b16 b15
        if s_14_13 {
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
        // D s_15_0: read-var pc:i
        let s_15_0: i128 = fn_state.pc;
        // D s_15_1: read-var opcode:u32
        let s_15_1: u32 = fn_state.opcode;
        // D s_15_2: call __DecodeA32_BranchBlock(s_15_0, s_15_1)
        let s_15_2: () = u__DecodeA32_BranchBlock(state, tracer, s_15_0, s_15_1);
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var pc:i
        let s_16_0: i128 = fn_state.pc;
        // D s_16_1: read-var opcode:u32
        let s_16_1: u32 = fn_state.opcode;
        // D s_16_2: call __DecodeA32_SysASIMDFP(s_16_0, s_16_1)
        let s_16_2: () = u__DecodeA32_SysASIMDFP(state, tracer, s_16_0, s_16_1);
        // N s_16_3: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #4s : i
        let s_17_0: i128 = 4;
        // D s_17_1: read-var v__40:u32
        let s_17_1: u32 = fn_state.v__40;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #0s : i
        let s_17_5: i128 = 0;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_0 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: bool = ((s_17_7.value()) != 0);
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 1u16);
        // C s_17_10: const #1u : u8
        let s_17_10: bool = true;
        // C s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 1u16);
        // D s_17_12: cmp-eq s_17_9 s_17_11
        let s_17_12: bool = ((s_17_9) == (s_17_11));
        // D s_17_13: write-var gs#428951 <= s_17_12
        fn_state.gs_428951 = s_17_12;
        // N s_17_14: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #4s : i
        let s_18_0: i128 = 4;
        // D s_18_1: read-var v__36:u32
        let s_18_1: u32 = fn_state.v__36;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 32u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #0s : i
        let s_18_5: i128 = 0;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_0 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: bool = ((s_18_7.value()) != 0);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 1u16);
        // C s_18_10: const #0u : u8
        let s_18_10: bool = false;
        // C s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 1u16);
        // D s_18_12: cmp-eq s_18_9 s_18_11
        let s_18_12: bool = ((s_18_9) == (s_18_11));
        // D s_18_13: write-var gs#428945 <= s_18_12
        fn_state.gs_428945 = s_18_12;
        // N s_18_14: jump b8
        return block_8(state, tracer, fn_state);
    }
}
