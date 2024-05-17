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
use AddrTop::*;
use UsingAArch32::*;
use IsInHost::*;
use u__id::*;
use zext_subrange::*;
use sext_subrange::*;
use common::*;
pub fn AArch64_BranchAddr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    el: u8,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_3229: bool,
        msbit: i128,
        gs_3226: bool,
        gs_3224: bool,
        return_value: u64,
        gs_3227: bool,
        vaddress: u64,
        el: u8,
    }
    let fn_state = FunctionState {
        vaddress,
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var vaddress:u64
        let s_0_4: u64 = fn_state.vaddress;
        // C s_0_5: const #1u : u8
        let s_0_5: bool = true;
        // D s_0_6: read-var el:u8
        let s_0_6: u8 = fn_state.el;
        // D s_0_7: call AddrTop(s_0_4, s_0_5, s_0_6)
        let s_0_7: i128 = AddrTop(state, tracer, s_0_4, s_0_5, s_0_6);
        // D s_0_8: write-var msbit <= s_0_7
        fn_state.msbit = s_0_7;
        // C s_0_9: const #63s : i
        let s_0_9: i128 = 63;
        // D s_0_10: read-var msbit:i
        let s_0_10: i128 = fn_state.msbit;
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // N s_0_12: branch s_0_11 b17 b1
        if s_0_11 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var msbit:i
        let s_1_0: i128 = fn_state.msbit;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: cmp-le s_1_2 s_1_1
        let s_1_3: bool = ((s_1_2) <= (s_1_1));
        // N s_1_4: branch s_1_3 b16 b2
        if s_1_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#3224 <= s_2_0
        fn_state.gs_3224 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var gs#3224:u8
        let s_3_0: bool = fn_state.gs_3224;
        // N s_3_1: assert s_3_0
        let s_3_1: () = assert!(s_3_0);
        // D s_3_2: read-var el:u8
        let s_3_2: u8 = fn_state.el;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #448u : u32
        let s_3_4: u32 = 448;
        // D s_3_5: read-reg s_3_4:u8
        let s_3_5: u8 = {
            let value = state.read_register::<u8>(s_3_4 as isize);
            tracer.read_register(s_3_4 as isize, value);
            value
        };
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 2u16);
        // D s_3_7: cmp-eq s_3_3 s_3_6
        let s_3_7: bool = ((s_3_3) == (s_3_6));
        // N s_3_8: branch s_3_7 b15 b4
        if s_3_7 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var el:u8
        let s_4_0: u8 = fn_state.el;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // D s_4_6: write-var gs#3226 <= s_4_5
        fn_state.gs_3226 = s_4_5;
        // N s_4_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var gs#3226:u8
        let s_5_0: bool = fn_state.gs_3226;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call IsInHost(s_6_0)
        let s_6_1: bool = IsInHost(state, tracer, s_6_0);
        // D s_6_2: write-var gs#3227 <= s_6_1
        fn_state.gs_3227 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#3227:u8
        let s_7_0: bool = fn_state.gs_3227;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#3229 <= s_8_0
        fn_state.gs_3229 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#3229:u8
        let s_9_0: bool = fn_state.gs_3229;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_10_0: const #64s : i
        let s_10_0: i128 = 64;
        // C s_10_1: const #0s : i
        let s_10_1: i128 = 0;
        // D s_10_2: read-var vaddress:u64
        let s_10_2: u64 = fn_state.vaddress;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: read-var msbit:i
        let s_10_4: i128 = fn_state.msbit;
        // D s_10_5: call zext_subrange(s_10_0, s_10_3, s_10_4, s_10_1)
        let s_10_5: Bits = zext_subrange(state, tracer, s_10_0, s_10_3, s_10_4, s_10_1);
        // D s_10_6: cast reint s_10_5 -> u64
        let s_10_6: u64 = (s_10_5.value() as u64);
        // D s_10_7: write-var return_value <= s_10_6
        fn_state.return_value = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var return_value:u64
        let s_11_0: u64 = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_12_0: const #64s : i
        let s_12_0: i128 = 64;
        // C s_12_1: const #0s : i
        let s_12_1: i128 = 0;
        // D s_12_2: read-var vaddress:u64
        let s_12_2: u64 = fn_state.vaddress;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 64u16);
        // D s_12_4: read-var msbit:i
        let s_12_4: i128 = fn_state.msbit;
        // D s_12_5: call sext_subrange(s_12_0, s_12_3, s_12_4, s_12_1)
        let s_12_5: Bits = sext_subrange(state, tracer, s_12_0, s_12_3, s_12_4, s_12_1);
        // D s_12_6: cast reint s_12_5 -> u64
        let s_12_6: u64 = (s_12_5.value() as u64);
        // D s_12_7: write-var return_value <= s_12_6
        fn_state.return_value = s_12_6;
        // N s_12_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var vaddress:u64
        let s_13_0: u64 = fn_state.vaddress;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 64u16);
        // D s_13_2: read-var msbit:i
        let s_13_2: i128 = fn_state.msbit;
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_1 s_13_2 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_1) >> (s_13_2)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#3229 <= s_13_21
        fn_state.gs_3229 = s_13_21;
        // N s_13_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#3227 <= s_14_0
        fn_state.gs_3227 = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#3226 <= s_15_0
        fn_state.gs_3226 = s_15_0;
        // N s_15_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var msbit:i
        let s_16_0: i128 = fn_state.msbit;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #64s : i
        let s_16_2: i128 = 64;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // D s_16_4: write-var gs#3224 <= s_16_3
        fn_state.gs_3224 = s_16_3;
        // N s_16_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var vaddress:u64
        let s_17_0: u64 = fn_state.vaddress;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b11
        return block_11(state, tracer, fn_state);
    }
}
