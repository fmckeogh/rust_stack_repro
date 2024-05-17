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
use ThisInstrAddr::*;
use UsingAArch32::*;
use HaveBTIExt::*;
use AArch64_BranchTargetException::*;
use AArch64_ExecutingBTIInstr::*;
use Halted::*;
use AArch64_ExecutingBROrBLROrRetInstr::*;
use common::*;
pub fn BranchTargetCheck<T: Tracer>(state: &mut State, tracer: &T, gs_6530: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        bti_instr: bool,
        gs_6534: bool,
        gs_6531: bool,
        gs_6532: bool,
        gs_6533: bool,
        gs_6540: bool,
        gs_6530: (),
    }
    let fn_state = FunctionState {
        gs_6530,
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
        // S s_0_1: call HaveBTIExt(s_0_0)
        let s_0_1: bool = HaveBTIExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b20 b1
        if s_0_1 {
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
        // D s_1_1: write-var gs#6531 <= s_1_0
        fn_state.gs_6531 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6531:u8
        let s_2_0: bool = fn_state.gs_6531;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #102520u : u32
        let s_2_2: u32 = 102520;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: bool = {
            let value = state.read_register::<bool>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // N s_2_4: branch s_2_3 b19 b3
        if s_2_3 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#6532 <= s_3_0
        fn_state.gs_6532 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#6532:u8
        let s_4_0: bool = fn_state.gs_6532;
        // N s_4_1: branch s_4_0 b18 b5
        if s_4_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#6533 <= s_5_0
        fn_state.gs_6533 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#6533:u8
        let s_6_0: bool = fn_state.gs_6533;
        // N s_6_1: branch s_6_0 b17 b7
        if s_6_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#6534 <= s_7_0
        fn_state.gs_6534 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#6534:u8
        let s_8_0: bool = fn_state.gs_6534;
        // N s_8_1: branch s_8_0 b16 b9
        if s_8_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call AArch64_ExecutingBROrBLROrRetInstr(s_10_0)
        let s_10_1: bool = AArch64_ExecutingBROrBLROrRetInstr(state, tracer, s_10_0);
        // C s_10_2: const #() : ()
        let s_10_2: () = ();
        // S s_10_3: call AArch64_ExecutingBTIInstr(s_10_2)
        let s_10_3: bool = AArch64_ExecutingBTIInstr(state, tracer, s_10_2);
        // D s_10_4: write-var bti_instr <= s_10_3
        fn_state.bti_instr = s_10_3;
        // N s_10_5: branch s_10_1 b15 b11
        if s_10_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var bti_instr:u8
        let s_11_0: bool = fn_state.bti_instr;
        // D s_11_1: write-var gs#6540 <= s_11_0
        fn_state.gs_6540 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#6540:u8
        let s_12_0: bool = fn_state.gs_6540;
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
        // N s_13_0: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: u8 = 0;
        // C s_14_1: const #90912u : u32
        let s_14_1: u32 = 90912;
        // N s_14_2: write-reg s_14_1 <= s_14_0
        let s_14_2: () = {
            state.write_register::<u8>(s_14_1 as isize, s_14_0);
            tracer.write_register(s_14_1 as isize, s_14_0);
        };
        // N s_14_3: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#6540 <= s_15_0
        fn_state.gs_6540 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // C s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // S s_16_2: call ThisInstrAddr(s_16_1)
        let s_16_2: Bits = ThisInstrAddr(state, tracer, s_16_1);
        // S s_16_3: cast reint s_16_2 -> u64
        let s_16_3: u64 = (s_16_2.value() as u64);
        // C s_16_4: const #0s : i
        let s_16_4: i128 = 0;
        // S s_16_5: cast zx s_16_3 -> bv
        let s_16_5: Bits = Bits::new(s_16_3 as u128, 64u16);
        // C s_16_6: const #1s : i64
        let s_16_6: i64 = 1;
        // C s_16_7: cast zx s_16_6 -> i
        let s_16_7: i128 = (i128::try_from(s_16_6).unwrap());
        // C s_16_8: const #51s : i
        let s_16_8: i128 = 51;
        // C s_16_9: add s_16_8 s_16_7
        let s_16_9: i128 = (s_16_8 + s_16_7);
        // D s_16_10: bit-extract s_16_5 s_16_4 s_16_9
        let s_16_10: Bits = (Bits::new(
            ((s_16_5) >> (s_16_4)).value(),
            u16::try_from(s_16_9).unwrap(),
        ));
        // D s_16_11: cast reint s_16_10 -> u52
        let s_16_11: u64 = (s_16_10.value() as u64);
        // D s_16_12: call AArch64_BranchTargetException(s_16_11)
        let s_16_12: () = AArch64_BranchTargetException(state, tracer, s_16_11);
        // N s_16_13: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Halted(s_17_0)
        let s_17_1: bool = Halted(state, tracer, s_17_0);
        // S s_17_2: not s_17_1
        let s_17_2: bool = !s_17_1;
        // D s_17_3: write-var gs#6534 <= s_17_2
        fn_state.gs_6534 = s_17_2;
        // N s_17_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #20952u : u32
        let s_18_0: u32 = 20952;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: bool = {
            let value = state.read_register::<bool>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // D s_18_3: write-var gs#6533 <= s_18_2
        fn_state.gs_6533 = s_18_2;
        // N s_18_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16970u : u32
        let s_19_0: u32 = 16970;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 2u16);
        // C s_19_3: const #0u : u8
        let s_19_3: u8 = 0;
        // C s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 2u16);
        // D s_19_5: cmp-ne s_19_2 s_19_4
        let s_19_5: bool = ((s_19_2) != (s_19_4));
        // D s_19_6: write-var gs#6532 <= s_19_5
        fn_state.gs_6532 = s_19_5;
        // N s_19_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call UsingAArch32(s_20_0)
        let s_20_1: bool = UsingAArch32(state, tracer, s_20_0);
        // S s_20_2: not s_20_1
        let s_20_2: bool = !s_20_1;
        // D s_20_3: write-var gs#6531 <= s_20_2
        fn_state.gs_6531 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
