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
use AArch32_GenerateDebugExceptions::*;
use AArch32_GenerateDebugExceptionsFrom::*;
use IllegalExceptionReturn::*;
use ELFromSPSR::*;
use Halted::*;
use ELUsingAArch32::*;
use UsingAArch32::*;
use AArch64_GenerateDebugExceptions::*;
use Restarting::*;
use SecurityStateAtEL::*;
use AArch64_GenerateDebugExceptionsFrom::*;
use DebugTargetFrom::*;
use u_get_MDSCR_EL1_Type_SS::*;
use common::*;
pub fn DebugExceptionReturnSS<T: Tracer>(
    state: &mut State,
    tracer: &T,
    spsr: Bits,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_5105: bool,
        gs_5120: bool,
        enabled_at_source: bool,
        dest_using_32: bool,
        dest_ss: u32,
        gs_5122: bool,
        gs_5121: bool,
        enabled_at_dest: bool,
        gs_5104: bool,
        dest_el: u8,
        ga_3441: ProductTypea5cc8de4daab131c,
        SS_bit: bool,
        spsr: Bits,
    }
    let fn_state = FunctionState {
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b32 b1
        if s_0_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Restarting(s_1_0)
        let s_1_1: bool = Restarting(state, tracer, s_1_0);
        // D s_1_2: write-var gs#5104 <= s_1_1
        fn_state.gs_5104 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#5104:u8
        let s_2_0: bool = fn_state.gs_5104;
        // N s_2_1: branch s_2_0 b31 b3
        if s_2_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #448u : u32
        let s_3_3: u32 = 448;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-ne s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) != (s_3_5));
        // D s_3_7: write-var gs#5105 <= s_3_6
        fn_state.gs_5105 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#5105:u8
        let s_4_0: bool = fn_state.gs_5105;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call Restarting(s_4_2)
        let s_4_3: bool = Restarting(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b30 b5
        if s_4_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call UsingAArch32(s_5_0)
        let s_5_1: bool = UsingAArch32(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b29 b6
        if s_5_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call AArch64_GenerateDebugExceptions(s_6_0)
        let s_6_1: bool = AArch64_GenerateDebugExceptions(state, tracer, s_6_0);
        // D s_6_2: write-var enabled_at_source <= s_6_1
        fn_state.enabled_at_source = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var spsr:bv
        let s_7_0: Bits = fn_state.spsr;
        // D s_7_1: call IllegalExceptionReturn(s_7_0)
        let s_7_1: bool = IllegalExceptionReturn(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b28 b8
        if s_7_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var spsr:bv
        let s_8_0: Bits = fn_state.spsr;
        // D s_8_1: call ELFromSPSR(s_8_0)
        let s_8_1: ProductTypea5cc8de4daab131c = ELFromSPSR(state, tracer, s_8_0);
        // D s_8_2: write-var ga#3441 <= s_8_1
        fn_state.ga_3441 = s_8_1;
        // D s_8_3: read-var ga#3441.0:struct
        let s_8_3: bool = fn_state.ga_3441._0;
        // D s_8_4: read-var ga#3441.1:struct
        let s_8_4: u8 = fn_state.ga_3441._1;
        // D s_8_5: write-var dest_el <= s_8_4
        fn_state.dest_el = s_8_4;
        // N s_8_6: assert s_8_3
        let s_8_6: () = assert!(s_8_3);
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var dest_el:u8
        let s_9_0: u8 = fn_state.dest_el;
        // D s_9_1: call SecurityStateAtEL(s_9_0)
        let s_9_1: u32 = SecurityStateAtEL(state, tracer, s_9_0);
        // D s_9_2: write-var dest_ss <= s_9_1
        fn_state.dest_ss = s_9_1;
        // D s_9_3: read-var dest_el:u8
        let s_9_3: u8 = fn_state.dest_el;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // C s_9_5: const #448u : u32
        let s_9_5: u32 = 448;
        // D s_9_6: read-reg s_9_5:u8
        let s_9_6: u8 = {
            let value = state.read_register::<u8>(s_9_5 as isize);
            tracer.read_register(s_9_5 as isize, value);
            value
        };
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 2u16);
        // D s_9_8: cmp-eq s_9_4 s_9_7
        let s_9_8: bool = ((s_9_4) == (s_9_7));
        // N s_9_9: branch s_9_8 b27 b10
        if s_9_8 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var dest_el:u8
        let s_10_0: u8 = fn_state.dest_el;
        // D s_10_1: call ELUsingAArch32(s_10_0)
        let s_10_1: bool = ELUsingAArch32(state, tracer, s_10_0);
        // D s_10_2: write-var dest_using_32 <= s_10_1
        fn_state.dest_using_32 = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var dest_using_32:u8
        let s_11_0: bool = fn_state.dest_using_32;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var spsr:bv
        let s_12_0: Bits = fn_state.spsr;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #9s : i
        let s_12_3: i128 = 9;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: assert s_12_4
        let s_12_5: () = assert!(s_12_4);
        // C s_12_6: const #9s : i
        let s_12_6: i128 = 9;
        // D s_12_7: read-var spsr:bv
        let s_12_7: Bits = fn_state.spsr;
        // C s_12_8: const #1u : u64
        let s_12_8: u64 = 1;
        // D s_12_9: bit-extract s_12_7 s_12_6 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_7) >> (s_12_6)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: bool = ((s_12_9.value()) != 0);
        // C s_12_11: const #0s : i
        let s_12_11: i128 = 0;
        // C s_12_12: const #0u : u64
        let s_12_12: u64 = 0;
        // D s_12_13: cast zx s_12_10 -> u64
        let s_12_13: u64 = (s_12_10 as u64);
        // C s_12_14: const #1u : u64
        let s_12_14: u64 = 1;
        // D s_12_15: and s_12_13 s_12_14
        let s_12_15: u64 = ((s_12_13) & (s_12_14));
        // D s_12_16: cmp-eq s_12_15 s_12_14
        let s_12_16: bool = ((s_12_15) == (s_12_14));
        // D s_12_17: lsl s_12_13 s_12_11
        let s_12_17: u64 = s_12_13 << s_12_11;
        // D s_12_18: or s_12_12 s_12_17
        let s_12_18: u64 = ((s_12_12) | (s_12_17));
        // D s_12_19: cmpl s_12_17
        let s_12_19: u64 = !s_12_17;
        // D s_12_20: and s_12_12 s_12_19
        let s_12_20: u64 = ((s_12_12) & (s_12_19));
        // D s_12_21: select s_12_16 s_12_18 s_12_20
        let s_12_21: u64 = if s_12_16 { s_12_18 } else { s_12_20 };
        // D s_12_22: cast trunc s_12_21 -> u8
        let s_12_22: bool = ((s_12_21) != 0);
        // D s_12_23: read-var dest_el:u8
        let s_12_23: u8 = fn_state.dest_el;
        // D s_12_24: read-var dest_ss:u32
        let s_12_24: u32 = fn_state.dest_ss;
        // D s_12_25: call AArch64_GenerateDebugExceptionsFrom(s_12_23, s_12_24, s_12_22)
        let s_12_25: bool = AArch64_GenerateDebugExceptionsFrom(
            state,
            tracer,
            s_12_23,
            s_12_24,
            s_12_22,
        );
        // D s_12_26: write-var enabled_at_dest <= s_12_25
        fn_state.enabled_at_dest = s_12_25;
        // N s_12_27: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var dest_ss:u32
        let s_13_0: u32 = fn_state.dest_ss;
        // D s_13_1: call DebugTargetFrom(s_13_0)
        let s_13_1: u8 = DebugTargetFrom(state, tracer, s_13_0);
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b25 b14
        if s_13_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#5120 <= s_14_0
        fn_state.gs_5120 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#5120:u8
        let s_15_0: bool = fn_state.gs_5120;
        // N s_15_1: branch s_15_0 b24 b16
        if s_15_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#5121 <= s_16_0
        fn_state.gs_5121 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var gs#5121:u8
        let s_17_0: bool = fn_state.gs_5121;
        // N s_17_1: branch s_17_0 b23 b18
        if s_17_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#5122 <= s_18_0
        fn_state.gs_5122 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var gs#5122:u8
        let s_19_0: bool = fn_state.gs_5122;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var SS_bit <= s_20_0
        fn_state.SS_bit = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var SS_bit:u8
        let s_21_0: bool = fn_state.SS_bit;
        // N s_21_1: return s_21_0
        return s_21_0;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var spsr:bv
        let s_22_0: Bits = fn_state.spsr;
        // D s_22_1: size-of s_22_0
        let s_22_1: u16 = s_22_0.length();
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // C s_22_3: const #21s : i
        let s_22_3: i128 = 21;
        // D s_22_4: cmp-lt s_22_3 s_22_2
        let s_22_4: bool = ((s_22_3) < (s_22_2));
        // N s_22_5: assert s_22_4
        let s_22_5: () = assert!(s_22_4);
        // C s_22_6: const #21s : i
        let s_22_6: i128 = 21;
        // D s_22_7: read-var spsr:bv
        let s_22_7: Bits = fn_state.spsr;
        // C s_22_8: const #1u : u64
        let s_22_8: u64 = 1;
        // D s_22_9: bit-extract s_22_7 s_22_6 s_22_8
        let s_22_9: Bits = (Bits::new(
            ((s_22_7) >> (s_22_6)).value(),
            u16::try_from(s_22_8).unwrap(),
        ));
        // D s_22_10: cast reint s_22_9 -> u8
        let s_22_10: bool = ((s_22_9.value()) != 0);
        // C s_22_11: const #0s : i
        let s_22_11: i128 = 0;
        // C s_22_12: const #0u : u64
        let s_22_12: u64 = 0;
        // D s_22_13: cast zx s_22_10 -> u64
        let s_22_13: u64 = (s_22_10 as u64);
        // C s_22_14: const #1u : u64
        let s_22_14: u64 = 1;
        // D s_22_15: and s_22_13 s_22_14
        let s_22_15: u64 = ((s_22_13) & (s_22_14));
        // D s_22_16: cmp-eq s_22_15 s_22_14
        let s_22_16: bool = ((s_22_15) == (s_22_14));
        // D s_22_17: lsl s_22_13 s_22_11
        let s_22_17: u64 = s_22_13 << s_22_11;
        // D s_22_18: or s_22_12 s_22_17
        let s_22_18: u64 = ((s_22_12) | (s_22_17));
        // D s_22_19: cmpl s_22_17
        let s_22_19: u64 = !s_22_17;
        // D s_22_20: and s_22_12 s_22_19
        let s_22_20: u64 = ((s_22_12) & (s_22_19));
        // D s_22_21: select s_22_16 s_22_18 s_22_20
        let s_22_21: u64 = if s_22_16 { s_22_18 } else { s_22_20 };
        // D s_22_22: cast trunc s_22_21 -> u8
        let s_22_22: bool = ((s_22_21) != 0);
        // D s_22_23: write-var SS_bit <= s_22_22
        fn_state.SS_bit = s_22_22;
        // N s_22_24: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var enabled_at_dest:u8
        let s_23_0: bool = fn_state.enabled_at_dest;
        // D s_23_1: write-var gs#5122 <= s_23_0
        fn_state.gs_5122 = s_23_0;
        // N s_23_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var enabled_at_source:u8
        let s_24_0: bool = fn_state.enabled_at_source;
        // D s_24_1: not s_24_0
        let s_24_1: bool = !s_24_0;
        // D s_24_2: write-var gs#5121 <= s_24_1
        fn_state.gs_5121 = s_24_1;
        // N s_24_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #104648u : u32
        let s_25_0: u32 = 104648;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_MDSCR_EL1_Type_SS(s_25_1)
        let s_25_2: bool = u_get_MDSCR_EL1_Type_SS(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: write-var gs#5120 <= s_25_6
        fn_state.gs_5120 = s_25_6;
        // N s_25_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var dest_el:u8
        let s_26_0: u8 = fn_state.dest_el;
        // D s_26_1: read-var dest_ss:u32
        let s_26_1: u32 = fn_state.dest_ss;
        // D s_26_2: call AArch32_GenerateDebugExceptionsFrom(s_26_0, s_26_1)
        let s_26_2: bool = AArch32_GenerateDebugExceptionsFrom(
            state,
            tracer,
            s_26_0,
            s_26_1,
        );
        // D s_26_3: write-var enabled_at_dest <= s_26_2
        fn_state.enabled_at_dest = s_26_2;
        // N s_26_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #4s : i
        let s_27_0: i128 = 4;
        // D s_27_1: read-var spsr:bv
        let s_27_1: Bits = fn_state.spsr;
        // C s_27_2: const #1u : u64
        let s_27_2: u64 = 1;
        // D s_27_3: bit-extract s_27_1 s_27_0 s_27_2
        let s_27_3: Bits = (Bits::new(
            ((s_27_1) >> (s_27_0)).value(),
            u16::try_from(s_27_2).unwrap(),
        ));
        // D s_27_4: cast reint s_27_3 -> u8
        let s_27_4: bool = ((s_27_3.value()) != 0);
        // C s_27_5: const #0s : i
        let s_27_5: i128 = 0;
        // C s_27_6: const #0u : u64
        let s_27_6: u64 = 0;
        // D s_27_7: cast zx s_27_4 -> u64
        let s_27_7: u64 = (s_27_4 as u64);
        // C s_27_8: const #1u : u64
        let s_27_8: u64 = 1;
        // D s_27_9: and s_27_7 s_27_8
        let s_27_9: u64 = ((s_27_7) & (s_27_8));
        // D s_27_10: cmp-eq s_27_9 s_27_8
        let s_27_10: bool = ((s_27_9) == (s_27_8));
        // D s_27_11: lsl s_27_7 s_27_5
        let s_27_11: u64 = s_27_7 << s_27_5;
        // D s_27_12: or s_27_6 s_27_11
        let s_27_12: u64 = ((s_27_6) | (s_27_11));
        // D s_27_13: cmpl s_27_11
        let s_27_13: u64 = !s_27_11;
        // D s_27_14: and s_27_6 s_27_13
        let s_27_14: u64 = ((s_27_6) & (s_27_13));
        // D s_27_15: select s_27_10 s_27_12 s_27_14
        let s_27_15: u64 = if s_27_10 { s_27_12 } else { s_27_14 };
        // D s_27_16: cast trunc s_27_15 -> u8
        let s_27_16: bool = ((s_27_15) != 0);
        // D s_27_17: cast zx s_27_16 -> bv
        let s_27_17: Bits = Bits::new(s_27_16 as u128, 1u16);
        // C s_27_18: const #1u : u8
        let s_27_18: bool = true;
        // C s_27_19: cast zx s_27_18 -> bv
        let s_27_19: Bits = Bits::new(s_27_18 as u128, 1u16);
        // D s_27_20: cmp-eq s_27_17 s_27_19
        let s_27_20: bool = ((s_27_17) == (s_27_19));
        // D s_27_21: write-var dest_using_32 <= s_27_20
        fn_state.dest_using_32 = s_27_20;
        // N s_27_22: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #16975u : u32
        let s_28_0: u32 = 16975;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: write-var dest_el <= s_28_1
        fn_state.dest_el = s_28_1;
        // N s_28_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call AArch32_GenerateDebugExceptions(s_29_0)
        let s_29_1: bool = AArch32_GenerateDebugExceptions(state, tracer, s_29_0);
        // D s_29_2: write-var enabled_at_source <= s_29_1
        fn_state.enabled_at_source = s_29_1;
        // N s_29_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var enabled_at_source <= s_30_0
        fn_state.enabled_at_source = s_30_0;
        // N s_30_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#5105 <= s_31_0
        fn_state.gs_5105 = s_31_0;
        // N s_31_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#5104 <= s_32_0
        fn_state.gs_5104 = s_32_0;
        // N s_32_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
