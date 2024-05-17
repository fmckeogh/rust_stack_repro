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
use u__DecodeA64_Unallocated2::*;
use u__DecodeA64_Reserved::*;
use u__DecodeA64_SVE::*;
use u__DecodeA64_LoadStore::*;
use u__DecodeA64_Unallocated1::*;
use u__DecodeA64_DataProcImm::*;
use u__DecodeA64_SME::*;
use u__DecodeA64_DataProcFPSIMD::*;
use u__DecodeA64_BranchExcSys::*;
use u__DecodeA64_DataProcReg::*;
use common::*;
pub fn u__DecodeA64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pc: i128,
    opcode: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        v__0: u32,
        gs_428925: bool,
        v__3: u32,
        v__21: u32,
        gs_428904: bool,
        gs_428898: bool,
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
        // D s_0_1: write-var v__0 <= s_0_0
        fn_state.v__0 = s_0_0;
        // C s_0_2: const #31s : i
        let s_0_2: i128 = 31;
        // D s_0_3: read-var v__0:u32
        let s_0_3: u32 = fn_state.v__0;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_2 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_2)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: bool = ((s_0_9.value()) != 0);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #0u : u8
        let s_0_12: bool = false;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b27 b1
        if s_0_14 {
            return block_27(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#428898 <= s_1_0
        fn_state.gs_428898 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#428898:u8
        let s_2_0: bool = fn_state.gs_428898;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
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
        // D s_3_2: call __DecodeA64_Reserved(s_3_0, s_3_1)
        let s_3_2: () = u__DecodeA64_Reserved(state, tracer, s_3_0, s_3_1);
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
        // D s_4_1: write-var v__3 <= s_4_0
        fn_state.v__3 = s_4_0;
        // C s_4_2: const #31s : i
        let s_4_2: i128 = 31;
        // D s_4_3: read-var v__3:u32
        let s_4_3: u32 = fn_state.v__3;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #0s : i
        let s_4_7: i128 = 0;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_2 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_2)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u8
        let s_4_10: bool = ((s_4_9.value()) != 0);
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 1u16);
        // C s_4_12: const #1u : u8
        let s_4_12: bool = true;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 1u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // N s_4_15: branch s_4_14 b26 b5
        if s_4_14 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#428904 <= s_5_0
        fn_state.gs_428904 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#428904:u8
        let s_6_0: bool = fn_state.gs_428904;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b8 b7
        if s_6_1 {
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
        // D s_7_0: read-var pc:i
        let s_7_0: i128 = fn_state.pc;
        // D s_7_1: read-var opcode:u32
        let s_7_1: u32 = fn_state.opcode;
        // D s_7_2: call __DecodeA64_SME(s_7_0, s_7_1)
        let s_7_2: () = u__DecodeA64_SME(state, tracer, s_7_0, s_7_1);
        // N s_7_3: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var opcode:u32
        let s_8_0: u32 = fn_state.opcode;
        // C s_8_1: const #25s : i
        let s_8_1: i128 = 25;
        // D s_8_2: cast zx s_8_0 -> bv
        let s_8_2: Bits = Bits::new(s_8_0 as u128, 32u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #3s : i
        let s_8_5: i128 = 3;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_1 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_1)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 4u16);
        // C s_8_10: const #1u : u8
        let s_8_10: u8 = 1;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 4u16);
        // D s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12: bool = ((s_8_9) == (s_8_11));
        // D s_8_13: not s_8_12
        let s_8_13: bool = !s_8_12;
        // N s_8_14: branch s_8_13 b10 b9
        if s_8_13 {
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
        // D s_9_2: call __DecodeA64_Unallocated1(s_9_0, s_9_1)
        let s_9_2: () = u__DecodeA64_Unallocated1(state, tracer, s_9_0, s_9_1);
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
        // C s_10_1: const #25s : i
        let s_10_1: i128 = 25;
        // D s_10_2: cast zx s_10_0 -> bv
        let s_10_2: Bits = Bits::new(s_10_0 as u128, 32u16);
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #3s : i
        let s_10_5: i128 = 3;
        // C s_10_6: add s_10_5 s_10_4
        let s_10_6: i128 = (s_10_5 + s_10_4);
        // D s_10_7: bit-extract s_10_2 s_10_1 s_10_6
        let s_10_7: Bits = (Bits::new(
            ((s_10_2) >> (s_10_1)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D s_10_8: cast reint s_10_7 -> u8
        let s_10_8: u8 = (s_10_7.value() as u8);
        // D s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 4u16);
        // C s_10_10: const #2u : u8
        let s_10_10: u8 = 2;
        // C s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 4u16);
        // D s_10_12: cmp-eq s_10_9 s_10_11
        let s_10_12: bool = ((s_10_9) == (s_10_11));
        // D s_10_13: not s_10_12
        let s_10_13: bool = !s_10_12;
        // N s_10_14: branch s_10_13 b12 b11
        if s_10_13 {
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
        // D s_11_0: read-var pc:i
        let s_11_0: i128 = fn_state.pc;
        // D s_11_1: read-var opcode:u32
        let s_11_1: u32 = fn_state.opcode;
        // D s_11_2: call __DecodeA64_SVE(s_11_0, s_11_1)
        let s_11_2: () = u__DecodeA64_SVE(state, tracer, s_11_0, s_11_1);
        // N s_11_3: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var opcode:u32
        let s_12_0: u32 = fn_state.opcode;
        // C s_12_1: const #25s : i
        let s_12_1: i128 = 25;
        // D s_12_2: cast zx s_12_0 -> bv
        let s_12_2: Bits = Bits::new(s_12_0 as u128, 32u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #3s : i
        let s_12_5: i128 = 3;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_1 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_1)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: u8 = (s_12_7.value() as u8);
        // D s_12_9: cast zx s_12_8 -> bv
        let s_12_9: Bits = Bits::new(s_12_8 as u128, 4u16);
        // C s_12_10: const #3u : u8
        let s_12_10: u8 = 3;
        // C s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 4u16);
        // D s_12_12: cmp-eq s_12_9 s_12_11
        let s_12_12: bool = ((s_12_9) == (s_12_11));
        // D s_12_13: not s_12_12
        let s_12_13: bool = !s_12_12;
        // N s_12_14: branch s_12_13 b14 b13
        if s_12_13 {
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
        // D s_13_2: call __DecodeA64_Unallocated2(s_13_0, s_13_1)
        let s_13_2: () = u__DecodeA64_Unallocated2(state, tracer, s_13_0, s_13_1);
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
        // C s_14_5: const #2s : i
        let s_14_5: i128 = 2;
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
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 3u16);
        // C s_14_10: const #4u : u8
        let s_14_10: u8 = 4;
        // C s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 3u16);
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
        // D s_15_2: call __DecodeA64_DataProcImm(s_15_0, s_15_1)
        let s_15_2: () = u__DecodeA64_DataProcImm(state, tracer, s_15_0, s_15_1);
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var opcode:u32
        let s_16_0: u32 = fn_state.opcode;
        // C s_16_1: const #26s : i
        let s_16_1: i128 = 26;
        // D s_16_2: cast zx s_16_0 -> bv
        let s_16_2: Bits = Bits::new(s_16_0 as u128, 32u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #2s : i
        let s_16_5: i128 = 2;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_1 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u8
        let s_16_8: u8 = (s_16_7.value() as u8);
        // D s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 3u16);
        // C s_16_10: const #5u : u8
        let s_16_10: u8 = 5;
        // C s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 3u16);
        // D s_16_12: cmp-eq s_16_9 s_16_11
        let s_16_12: bool = ((s_16_9) == (s_16_11));
        // D s_16_13: not s_16_12
        let s_16_13: bool = !s_16_12;
        // N s_16_14: branch s_16_13 b18 b17
        if s_16_13 {
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
        // D s_17_0: read-var pc:i
        let s_17_0: i128 = fn_state.pc;
        // D s_17_1: read-var opcode:u32
        let s_17_1: u32 = fn_state.opcode;
        // D s_17_2: call __DecodeA64_BranchExcSys(s_17_0, s_17_1)
        let s_17_2: () = u__DecodeA64_BranchExcSys(state, tracer, s_17_0, s_17_1);
        // N s_17_3: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var opcode:u32
        let s_18_0: u32 = fn_state.opcode;
        // D s_18_1: write-var v__21 <= s_18_0
        fn_state.v__21 = s_18_0;
        // C s_18_2: const #27s : i
        let s_18_2: i128 = 27;
        // D s_18_3: read-var v__21:u32
        let s_18_3: u32 = fn_state.v__21;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 32u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #0s : i
        let s_18_7: i128 = 0;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_2 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: bool = ((s_18_9.value()) != 0);
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 1u16);
        // C s_18_12: const #1u : u8
        let s_18_12: bool = true;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 1u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // N s_18_15: branch s_18_14 b25 b19
        if s_18_14 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#428925 <= s_19_0
        fn_state.gs_428925 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#428925:u8
        let s_20_0: bool = fn_state.gs_428925;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b22 b21
        if s_20_1 {
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
        // D s_21_0: read-var pc:i
        let s_21_0: i128 = fn_state.pc;
        // D s_21_1: read-var opcode:u32
        let s_21_1: u32 = fn_state.opcode;
        // D s_21_2: call __DecodeA64_LoadStore(s_21_0, s_21_1)
        let s_21_2: () = u__DecodeA64_LoadStore(state, tracer, s_21_0, s_21_1);
        // N s_21_3: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var opcode:u32
        let s_22_0: u32 = fn_state.opcode;
        // C s_22_1: const #25s : i
        let s_22_1: i128 = 25;
        // D s_22_2: cast zx s_22_0 -> bv
        let s_22_2: Bits = Bits::new(s_22_0 as u128, 32u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #2s : i
        let s_22_5: i128 = 2;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_1 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_1)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 3u16);
        // C s_22_10: const #5u : u8
        let s_22_10: u8 = 5;
        // C s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 3u16);
        // D s_22_12: cmp-eq s_22_9 s_22_11
        let s_22_12: bool = ((s_22_9) == (s_22_11));
        // D s_22_13: not s_22_12
        let s_22_13: bool = !s_22_12;
        // N s_22_14: branch s_22_13 b24 b23
        if s_22_13 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var pc:i
        let s_23_0: i128 = fn_state.pc;
        // D s_23_1: read-var opcode:u32
        let s_23_1: u32 = fn_state.opcode;
        // D s_23_2: call __DecodeA64_DataProcReg(s_23_0, s_23_1)
        let s_23_2: () = u__DecodeA64_DataProcReg(state, tracer, s_23_0, s_23_1);
        // N s_23_3: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var pc:i
        let s_24_0: i128 = fn_state.pc;
        // D s_24_1: read-var opcode:u32
        let s_24_1: u32 = fn_state.opcode;
        // D s_24_2: call __DecodeA64_DataProcFPSIMD(s_24_0, s_24_1)
        let s_24_2: () = u__DecodeA64_DataProcFPSIMD(state, tracer, s_24_0, s_24_1);
        // N s_24_3: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #25s : i
        let s_25_0: i128 = 25;
        // D s_25_1: read-var v__21:u32
        let s_25_1: u32 = fn_state.v__21;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 32u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #0s : i
        let s_25_5: i128 = 0;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_0 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: bool = ((s_25_7.value()) != 0);
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 1u16);
        // C s_25_10: const #0u : u8
        let s_25_10: bool = false;
        // C s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 1u16);
        // D s_25_12: cmp-eq s_25_9 s_25_11
        let s_25_12: bool = ((s_25_9) == (s_25_11));
        // D s_25_13: write-var gs#428925 <= s_25_12
        fn_state.gs_428925 = s_25_12;
        // N s_25_14: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #25s : i
        let s_26_0: i128 = 25;
        // D s_26_1: read-var v__3:u32
        let s_26_1: u32 = fn_state.v__3;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 32u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #3s : i
        let s_26_5: i128 = 3;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: u8 = (s_26_7.value() as u8);
        // D s_26_9: cast zx s_26_8 -> bv
        let s_26_9: Bits = Bits::new(s_26_8 as u128, 4u16);
        // C s_26_10: const #0u : u8
        let s_26_10: u8 = 0;
        // C s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 4u16);
        // D s_26_12: cmp-eq s_26_9 s_26_11
        let s_26_12: bool = ((s_26_9) == (s_26_11));
        // D s_26_13: write-var gs#428904 <= s_26_12
        fn_state.gs_428904 = s_26_12;
        // N s_26_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #25s : i
        let s_27_0: i128 = 25;
        // D s_27_1: read-var v__0:u32
        let s_27_1: u32 = fn_state.v__0;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 32u16);
        // C s_27_3: const #1s : i64
        let s_27_3: i64 = 1;
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #3s : i
        let s_27_5: i128 = 3;
        // C s_27_6: add s_27_5 s_27_4
        let s_27_6: i128 = (s_27_5 + s_27_4);
        // D s_27_7: bit-extract s_27_2 s_27_0 s_27_6
        let s_27_7: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_6).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: u8 = (s_27_7.value() as u8);
        // D s_27_9: cast zx s_27_8 -> bv
        let s_27_9: Bits = Bits::new(s_27_8 as u128, 4u16);
        // C s_27_10: const #0u : u8
        let s_27_10: u8 = 0;
        // C s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 4u16);
        // D s_27_12: cmp-eq s_27_9 s_27_11
        let s_27_12: bool = ((s_27_9) == (s_27_11));
        // D s_27_13: write-var gs#428898 <= s_27_12
        fn_state.gs_428898 = s_27_12;
        // N s_27_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
