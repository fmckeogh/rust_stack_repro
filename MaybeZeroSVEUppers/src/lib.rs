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
use zext_subrange::*;
use fdiv_int::*;
use CurrentSVL_read::*;
use ConstrainUnpredictableBool::*;
use CurrentVL_read::*;
use Zeros::*;
use HaveSME::*;
use u__id::*;
use ELUsingAArch32::*;
use IsFPEnabled::*;
use MaxImplementedSVL::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use IsSVEEnabled::*;
use common::*;
pub fn MaybeZeroSVEUppers<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_3918: i64,
        gs_5944: i64,
        VLshadow_69: i64,
        n: i64,
        u_206: i64,
        u_207: i64,
        gs_5908: bool,
        gs_5954: i64,
        gs_5960: bool,
        PL: i64,
        gs_5934: bool,
        u_208: i64,
        gs_5974: bool,
        VL: i64,
        SVLshadow_70: i64,
        SVL: i64,
        allvecs: i128,
        lower_enabled: bool,
        accessiblevecs: i64,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
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
        // S s_0_1: call CurrentSVL_read(s_0_0)
        let s_0_1: i64 = CurrentSVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var SVL <= s_0_1
        fn_state.SVL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CurrentVL_read(s_0_3)
        let s_0_4: i64 = CurrentVL_read(state, tracer, s_0_3);
        // D s_0_5: write-var VL <= s_0_4
        fn_state.VL = s_0_4;
        // D s_0_6: read-var target_el:u8
        let s_0_6: u8 = fn_state.target_el;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // C s_0_10: const #16975u : u32
        let s_0_10: u32 = 16975;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: cast zx s_0_9 -> i
        let s_0_15: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_16: cast zx s_0_14 -> i
        let s_0_16: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_17: cmp-le s_0_15 s_0_16
        let s_0_17: bool = ((s_0_15) <= (s_0_16));
        // N s_0_18: branch s_0_17 b57 b1
        if s_0_17 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var target_el:u8
        let s_1_0: u8 = fn_state.target_el;
        // D s_1_1: call IsSVEEnabled(s_1_0)
        let s_1_1: bool = IsSVEEnabled(state, tracer, s_1_0);
        // D s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#5908 <= s_1_2
        fn_state.gs_5908 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#5908:u8
        let s_2_0: bool = fn_state.gs_5908;
        // N s_2_1: branch s_2_0 b56 b3
        if s_2_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var target_el:u8
        let s_3_0: u8 = fn_state.target_el;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b53 b4
        if s_3_5 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var target_el:u8
        let s_4_0: u8 = fn_state.target_el;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
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
        // N s_4_6: branch s_4_5 b50 b5
        if s_4_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var target_el:u8
        let s_5_0: u8 = fn_state.target_el;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #440u : u32
        let s_5_2: u32 = 440;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b49 b6
        if s_5_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#5974 <= s_6_0
        fn_state.gs_5974 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#5974:u8
        let s_7_0: bool = fn_state.gs_5974;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // C s_7_2: const #448u : u32
        let s_7_2: u32 = 448;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: call IsFPEnabled(s_7_3)
        let s_7_4: bool = IsFPEnabled(state, tracer, s_7_3);
        // D s_7_5: write-var lower_enabled <= s_7_4
        fn_state.lower_enabled = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var lower_enabled:u8
        let s_8_0: bool = fn_state.lower_enabled;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call IsSVEEnabled(s_10_1)
        let s_10_2: bool = IsSVEEnabled(state, tracer, s_10_1);
        // N s_10_3: branch s_10_2 b48 b11
        if s_10_2 {
            return block_48(state, tracer, fn_state);
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
        // D s_11_1: write-var ga#3918 <= s_11_0
        fn_state.ga_3918 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#3918:i64
        let s_12_0: i64 = fn_state.ga_3918;
        // D s_12_1: write-var VLshadow#69 <= s_12_0
        fn_state.VLshadow_69 = s_12_0;
        // C s_12_2: const #8s : i
        let s_12_2: i128 = 8;
        // D s_12_3: read-var VLshadow#69:i64
        let s_12_3: i64 = fn_state.VLshadow_69;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: div s_12_4 s_12_2
        let s_12_5: i128 = ((s_12_4) / (s_12_2));
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: write-var PL <= s_12_6
        fn_state.PL = s_12_6;
        // C s_12_8: const #0s : i64
        let s_12_8: i64 = 0;
        // D s_12_9: write-var n <= s_12_8
        fn_state.n = s_12_8;
        // N s_12_10: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // C s_13_1: const #31s : i64
        let s_13_1: i64 = 31;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b18 b14
        if s_13_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #50u : u32
        let s_14_0: u32 = 50;
        // S s_14_1: call ConstrainUnpredictableBool(s_14_0)
        let s_14_1: bool = ConstrainUnpredictableBool(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b17 b15
        if s_14_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // C s_16_1: const #1s : i64
        let s_16_1: i64 = 1;
        // D s_16_2: add s_16_0 s_16_1
        let s_16_2: i64 = (s_16_0 + s_16_1);
        // D s_16_3: write-var n <= s_16_2
        fn_state.n = s_16_2;
        // N s_16_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1800u : u32
        let s_17_0: u32 = 1800;
        // D s_17_1: read-reg s_17_0:[u2048; 32]
        let s_17_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: read-var n:i64
        let s_17_2: i64 = fn_state.n;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-element s_17_1[s_17_3]
        let s_17_4: u64 = s_17_1[(s_17_3) as usize];
        // C s_17_5: const #1s : i
        let s_17_5: i128 = 1;
        // D s_17_6: read-var VLshadow#69:i64
        let s_17_6: i64 = fn_state.VLshadow_69;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // D s_17_8: sub s_17_7 s_17_5
        let s_17_8: i128 = ((s_17_7) - (s_17_5));
        // D s_17_9: cast reint s_17_8 -> i64
        let s_17_9: i64 = (s_17_8 as i64);
        // C s_17_10: const #0s : i
        let s_17_10: i128 = 0;
        // C s_17_11: const #808u : u32
        let s_17_11: u32 = 808;
        // D s_17_12: read-reg s_17_11:i64
        let s_17_12: i64 = {
            let value = state.read_register::<i64>(s_17_11 as isize);
            tracer.read_register(s_17_11 as isize, value);
            value
        };
        // D s_17_13: cast zx s_17_12 -> i
        let s_17_13: i128 = (i128::try_from(s_17_12).unwrap());
        // D s_17_14: cast zx s_17_4 -> bv
        let s_17_14: Bits = Bits::new(s_17_4 as u128, 2048u16);
        // D s_17_15: cast zx s_17_9 -> i
        let s_17_15: i128 = (i128::try_from(s_17_9).unwrap());
        // D s_17_16: call zext_subrange(s_17_13, s_17_14, s_17_15, s_17_10)
        let s_17_16: Bits = zext_subrange(
            state,
            tracer,
            s_17_13,
            s_17_14,
            s_17_15,
            s_17_10,
        );
        // D s_17_17: cast reint s_17_16 -> u2048
        let s_17_17: u64 = (s_17_16.value() as u64);
        // C s_17_18: const #1800u : u32
        let s_17_18: u32 = 1800;
        // D s_17_19: read-reg s_17_18:[u2048; 32]
        let s_17_19: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_17_18 as isize);
            tracer.read_register(s_17_18 as isize, value);
            value
        };
        // D s_17_20: read-var n:i64
        let s_17_20: i64 = fn_state.n;
        // D s_17_21: cast zx s_17_20 -> i
        let s_17_21: i128 = (i128::try_from(s_17_20).unwrap());
        // D s_17_22: mutate-element s_17_19[s_17_21] <= s_17_17
        let s_17_22: [u64; 32usize] = {
            let mut local = s_17_19.clone();
            local[(s_17_21) as usize] = s_17_17;
            local
        };
        // D s_17_23: cast cvt s_17_22 -> [u2048; 0]
        let s_17_23: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_17_22);
        // D s_17_24: cast cvt s_17_23 -> [u2048; 32]
        let s_17_24: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_17_23);
            buf
        };
        // C s_17_25: const #1800u : u32
        let s_17_25: u32 = 1800;
        // N s_17_26: write-reg s_17_25 <= s_17_24
        let s_17_26: () = {
            state.write_register::<[u64; 32usize]>(s_17_25 as isize, s_17_24);
            tracer.write_register(s_17_25 as isize, s_17_24);
        };
        // N s_17_27: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i64
        let s_18_0: i64 = 0;
        // D s_18_1: write-var u#206 <= s_18_0
        fn_state.u_206 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var u#206:i64
        let s_19_0: i64 = fn_state.u_206;
        // C s_19_1: const #15s : i64
        let s_19_1: i64 = 15;
        // D s_19_2: cmp-gt s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) > (s_19_1));
        // N s_19_3: branch s_19_2 b24 b20
        if s_19_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #50u : u32
        let s_20_0: u32 = 50;
        // S s_20_1: call ConstrainUnpredictableBool(s_20_0)
        let s_20_1: bool = ConstrainUnpredictableBool(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b23 b21
        if s_20_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var u#206:i64
        let s_22_0: i64 = fn_state.u_206;
        // C s_22_1: const #1s : i64
        let s_22_1: i64 = 1;
        // D s_22_2: add s_22_0 s_22_1
        let s_22_2: i64 = (s_22_0 + s_22_1);
        // D s_22_3: write-var u#206 <= s_22_2
        fn_state.u_206 = s_22_2;
        // N s_22_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #17736u : u32
        let s_23_0: u32 = 17736;
        // D s_23_1: read-reg s_23_0:[u256; 16]
        let s_23_1: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: read-var u#206:i64
        let s_23_2: i64 = fn_state.u_206;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: read-element s_23_1[s_23_3]
        let s_23_4: u64 = s_23_1[(s_23_3) as usize];
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // D s_23_6: read-var PL:i64
        let s_23_6: i64 = fn_state.PL;
        // D s_23_7: cast zx s_23_6 -> i
        let s_23_7: i128 = (i128::try_from(s_23_6).unwrap());
        // D s_23_8: sub s_23_7 s_23_5
        let s_23_8: i128 = ((s_23_7) - (s_23_5));
        // D s_23_9: cast reint s_23_8 -> i64
        let s_23_9: i64 = (s_23_8 as i64);
        // C s_23_10: const #0s : i
        let s_23_10: i128 = 0;
        // C s_23_11: const #816u : u32
        let s_23_11: u32 = 816;
        // D s_23_12: read-reg s_23_11:i64
        let s_23_12: i64 = {
            let value = state.read_register::<i64>(s_23_11 as isize);
            tracer.read_register(s_23_11 as isize, value);
            value
        };
        // D s_23_13: cast zx s_23_12 -> i
        let s_23_13: i128 = (i128::try_from(s_23_12).unwrap());
        // D s_23_14: cast zx s_23_4 -> bv
        let s_23_14: Bits = Bits::new(s_23_4 as u128, 256u16);
        // D s_23_15: cast zx s_23_9 -> i
        let s_23_15: i128 = (i128::try_from(s_23_9).unwrap());
        // D s_23_16: call zext_subrange(s_23_13, s_23_14, s_23_15, s_23_10)
        let s_23_16: Bits = zext_subrange(
            state,
            tracer,
            s_23_13,
            s_23_14,
            s_23_15,
            s_23_10,
        );
        // D s_23_17: cast reint s_23_16 -> u256
        let s_23_17: u64 = (s_23_16.value() as u64);
        // C s_23_18: const #17736u : u32
        let s_23_18: u32 = 17736;
        // D s_23_19: read-reg s_23_18:[u256; 16]
        let s_23_19: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_23_18 as isize);
            tracer.read_register(s_23_18 as isize, value);
            value
        };
        // D s_23_20: read-var u#206:i64
        let s_23_20: i64 = fn_state.u_206;
        // D s_23_21: cast zx s_23_20 -> i
        let s_23_21: i128 = (i128::try_from(s_23_20).unwrap());
        // D s_23_22: mutate-element s_23_19[s_23_21] <= s_23_17
        let s_23_22: [u64; 16usize] = {
            let mut local = s_23_19.clone();
            local[(s_23_21) as usize] = s_23_17;
            local
        };
        // D s_23_23: cast cvt s_23_22 -> [u256; 0]
        let s_23_23: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_23_22);
        // D s_23_24: cast cvt s_23_23 -> [u256; 16]
        let s_23_24: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_23_23);
            buf
        };
        // C s_23_25: const #17736u : u32
        let s_23_25: u32 = 17736;
        // N s_23_26: write-reg s_23_25 <= s_23_24
        let s_23_26: () = {
            state.write_register::<[u64; 16usize]>(s_23_25 as isize, s_23_24);
            tracer.write_register(s_23_25 as isize, s_23_24);
        };
        // N s_23_27: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #50u : u32
        let s_24_0: u32 = 50;
        // S s_24_1: call ConstrainUnpredictableBool(s_24_0)
        let s_24_1: bool = ConstrainUnpredictableBool(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b47 b25
        if s_24_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
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
        // N s_26_2: branch s_26_1 b46 b27
        if s_26_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#5934 <= s_27_0
        fn_state.gs_5934 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#5934:u8
        let s_28_0: bool = fn_state.gs_5934;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var SVL:i64
        let s_30_0: i64 = fn_state.SVL;
        // D s_30_1: write-var SVLshadow#70 <= s_30_0
        fn_state.SVLshadow_70 = s_30_0;
        // C s_30_2: const #8s : i
        let s_30_2: i128 = 8;
        // D s_30_3: read-var SVLshadow#70:i64
        let s_30_3: i64 = fn_state.SVLshadow_70;
        // D s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_5: div s_30_4 s_30_2
        let s_30_5: i128 = ((s_30_4) / (s_30_2));
        // D s_30_6: cast reint s_30_5 -> i64
        let s_30_6: i64 = (s_30_5 as i64);
        // D s_30_7: write-var accessiblevecs <= s_30_6
        fn_state.accessiblevecs = s_30_6;
        // C s_30_8: const #() : ()
        let s_30_8: () = ();
        // S s_30_9: call MaxImplementedSVL(s_30_8)
        let s_30_9: i128 = MaxImplementedSVL(state, tracer, s_30_8);
        // C s_30_10: const #8s : i
        let s_30_10: i128 = 8;
        // S s_30_11: call fdiv_int(s_30_9, s_30_10)
        let s_30_11: i128 = fdiv_int(state, tracer, s_30_9, s_30_10);
        // D s_30_12: write-var allvecs <= s_30_11
        fn_state.allvecs = s_30_11;
        // C s_30_13: const #0s : i64
        let s_30_13: i64 = 0;
        // C s_30_14: const #1s : i
        let s_30_14: i128 = 1;
        // D s_30_15: read-var accessiblevecs:i64
        let s_30_15: i64 = fn_state.accessiblevecs;
        // D s_30_16: cast zx s_30_15 -> i
        let s_30_16: i128 = (i128::try_from(s_30_15).unwrap());
        // D s_30_17: sub s_30_16 s_30_14
        let s_30_17: i128 = ((s_30_16) - (s_30_14));
        // D s_30_18: cast reint s_30_17 -> i64
        let s_30_18: i64 = (s_30_17 as i64);
        // D s_30_19: write-var gs#5944 <= s_30_18
        fn_state.gs_5944 = s_30_18;
        // D s_30_20: write-var u#207 <= s_30_13
        fn_state.u_207 = s_30_13;
        // N s_30_21: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var u#207:i64
        let s_31_0: i64 = fn_state.u_207;
        // D s_31_1: read-var gs#5944:i64
        let s_31_1: i64 = fn_state.gs_5944;
        // D s_31_2: cmp-gt s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) > (s_31_1));
        // N s_31_3: branch s_31_2 b36 b32
        if s_31_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #54u : u32
        let s_32_0: u32 = 54;
        // S s_32_1: call ConstrainUnpredictableBool(s_32_0)
        let s_32_1: bool = ConstrainUnpredictableBool(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b35 b33
        if s_32_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var u#207:i64
        let s_34_0: i64 = fn_state.u_207;
        // C s_34_1: const #1s : i64
        let s_34_1: i64 = 1;
        // D s_34_2: add s_34_0 s_34_1
        let s_34_2: i64 = (s_34_0 + s_34_1);
        // D s_34_3: write-var u#207 <= s_34_2
        fn_state.u_207 = s_34_2;
        // N s_34_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #23952u : u32
        let s_35_0: u32 = 23952;
        // D s_35_1: read-reg s_35_0:[u2048; 256]
        let s_35_1: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: read-var u#207:i64
        let s_35_2: i64 = fn_state.u_207;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: read-element s_35_1[s_35_3]
        let s_35_4: u64 = s_35_1[(s_35_3) as usize];
        // C s_35_5: const #1s : i
        let s_35_5: i128 = 1;
        // D s_35_6: read-var SVLshadow#70:i64
        let s_35_6: i64 = fn_state.SVLshadow_70;
        // D s_35_7: cast zx s_35_6 -> i
        let s_35_7: i128 = (i128::try_from(s_35_6).unwrap());
        // D s_35_8: sub s_35_7 s_35_5
        let s_35_8: i128 = ((s_35_7) - (s_35_5));
        // D s_35_9: cast reint s_35_8 -> i64
        let s_35_9: i64 = (s_35_8 as i64);
        // C s_35_10: const #0s : i
        let s_35_10: i128 = 0;
        // C s_35_11: const #808u : u32
        let s_35_11: u32 = 808;
        // D s_35_12: read-reg s_35_11:i64
        let s_35_12: i64 = {
            let value = state.read_register::<i64>(s_35_11 as isize);
            tracer.read_register(s_35_11 as isize, value);
            value
        };
        // D s_35_13: cast zx s_35_12 -> i
        let s_35_13: i128 = (i128::try_from(s_35_12).unwrap());
        // D s_35_14: cast zx s_35_4 -> bv
        let s_35_14: Bits = Bits::new(s_35_4 as u128, 2048u16);
        // D s_35_15: cast zx s_35_9 -> i
        let s_35_15: i128 = (i128::try_from(s_35_9).unwrap());
        // D s_35_16: call zext_subrange(s_35_13, s_35_14, s_35_15, s_35_10)
        let s_35_16: Bits = zext_subrange(
            state,
            tracer,
            s_35_13,
            s_35_14,
            s_35_15,
            s_35_10,
        );
        // D s_35_17: cast reint s_35_16 -> u2048
        let s_35_17: u64 = (s_35_16.value() as u64);
        // C s_35_18: const #23952u : u32
        let s_35_18: u32 = 23952;
        // D s_35_19: read-reg s_35_18:[u2048; 256]
        let s_35_19: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_35_18 as isize);
            tracer.read_register(s_35_18 as isize, value);
            value
        };
        // D s_35_20: read-var u#207:i64
        let s_35_20: i64 = fn_state.u_207;
        // D s_35_21: cast zx s_35_20 -> i
        let s_35_21: i128 = (i128::try_from(s_35_20).unwrap());
        // D s_35_22: mutate-element s_35_19[s_35_21] <= s_35_17
        let s_35_22: [u64; 256usize] = {
            let mut local = s_35_19.clone();
            local[(s_35_21) as usize] = s_35_17;
            local
        };
        // D s_35_23: cast cvt s_35_22 -> [u2048; 0]
        let s_35_23: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_35_22);
        // D s_35_24: cast cvt s_35_23 -> [u2048; 256]
        let s_35_24: [u64; 256usize] = {
            let mut buf = [Default::default(); 256usize];
            buf.copy_from_slice(&s_35_23);
            buf
        };
        // C s_35_25: const #23952u : u32
        let s_35_25: u32 = 23952;
        // N s_35_26: write-reg s_35_25 <= s_35_24
        let s_35_26: () = {
            state.write_register::<[u64; 256usize]>(s_35_25 as isize, s_35_24);
            tracer.write_register(s_35_25 as isize, s_35_24);
        };
        // N s_35_27: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var accessiblevecs:i64
        let s_36_0: i64 = fn_state.accessiblevecs;
        // C s_36_1: const #1s : i
        let s_36_1: i128 = 1;
        // D s_36_2: read-var allvecs:i
        let s_36_2: i128 = fn_state.allvecs;
        // D s_36_3: sub s_36_2 s_36_1
        let s_36_3: i128 = ((s_36_2) - (s_36_1));
        // D s_36_4: cast reint s_36_3 -> i64
        let s_36_4: i64 = (s_36_3 as i64);
        // D s_36_5: write-var gs#5954 <= s_36_4
        fn_state.gs_5954 = s_36_4;
        // D s_36_6: write-var u#208 <= s_36_0
        fn_state.u_208 = s_36_0;
        // N s_36_7: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var u#208:i64
        let s_37_0: i64 = fn_state.u_208;
        // D s_37_1: read-var gs#5954:i64
        let s_37_1: i64 = fn_state.gs_5954;
        // D s_37_2: cmp-gt s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) > (s_37_1));
        // N s_37_3: branch s_37_2 b45 b38
        if s_37_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #54u : u32
        let s_38_0: u32 = 54;
        // S s_38_1: call ConstrainUnpredictableBool(s_38_0)
        let s_38_1: bool = ConstrainUnpredictableBool(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b41 b39
        if s_38_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var u#208:i64
        let s_40_0: i64 = fn_state.u_208;
        // C s_40_1: const #1s : i64
        let s_40_1: i64 = 1;
        // D s_40_2: add s_40_0 s_40_1
        let s_40_2: i64 = (s_40_0 + s_40_1);
        // D s_40_3: write-var u#208 <= s_40_2
        fn_state.u_208 = s_40_2;
        // N s_40_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var u#208:i64
        let s_41_0: i64 = fn_state.u_208;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // C s_41_3: const #0s : i
        let s_41_3: i128 = 0;
        // D s_41_4: cmp-le s_41_3 s_41_2
        let s_41_4: bool = ((s_41_3) <= (s_41_2));
        // N s_41_5: branch s_41_4 b44 b42
        if s_41_4 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#5960 <= s_42_0
        fn_state.gs_5960 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#5960:u8
        let s_43_0: bool = fn_state.gs_5960;
        // N s_43_1: assert s_43_0
        let s_43_1: () = assert!(s_43_0);
        // C s_43_2: const #808u : u32
        let s_43_2: u32 = 808;
        // D s_43_3: read-reg s_43_2:i64
        let s_43_3: i64 = {
            let value = state.read_register::<i64>(s_43_2 as isize);
            tracer.read_register(s_43_2 as isize, value);
            value
        };
        // D s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_5: call Zeros(s_43_4)
        let s_43_5: Bits = Zeros(state, tracer, s_43_4);
        // D s_43_6: cast reint s_43_5 -> u2048
        let s_43_6: u64 = (s_43_5.value() as u64);
        // C s_43_7: const #23952u : u32
        let s_43_7: u32 = 23952;
        // D s_43_8: read-reg s_43_7:[u2048; 256]
        let s_43_8: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_43_7 as isize);
            tracer.read_register(s_43_7 as isize, value);
            value
        };
        // D s_43_9: read-var u#208:i64
        let s_43_9: i64 = fn_state.u_208;
        // D s_43_10: cast zx s_43_9 -> i
        let s_43_10: i128 = (i128::try_from(s_43_9).unwrap());
        // D s_43_11: mutate-element s_43_8[s_43_10] <= s_43_6
        let s_43_11: [u64; 256usize] = {
            let mut local = s_43_8.clone();
            local[(s_43_10) as usize] = s_43_6;
            local
        };
        // D s_43_12: cast cvt s_43_11 -> [u2048; 0]
        let s_43_12: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_43_11);
        // D s_43_13: cast cvt s_43_12 -> [u2048; 256]
        let s_43_13: [u64; 256usize] = {
            let mut buf = [Default::default(); 256usize];
            buf.copy_from_slice(&s_43_12);
            buf
        };
        // C s_43_14: const #23952u : u32
        let s_43_14: u32 = 23952;
        // N s_43_15: write-reg s_43_14 <= s_43_13
        let s_43_15: () = {
            state.write_register::<[u64; 256usize]>(s_43_14 as isize, s_43_13);
            tracer.write_register(s_43_14 as isize, s_43_13);
        };
        // N s_43_16: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var u#208:i64
        let s_44_0: i64 = fn_state.u_208;
        // D s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_2: call __id(s_44_1)
        let s_44_2: i128 = u__id(state, tracer, s_44_1);
        // C s_44_3: const #256s : i
        let s_44_3: i128 = 256;
        // D s_44_4: cmp-lt s_44_2 s_44_3
        let s_44_4: bool = ((s_44_2) < (s_44_3));
        // D s_44_5: write-var gs#5960 <= s_44_4
        fn_state.gs_5960 = s_44_4;
        // N s_44_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #16998u : u32
        let s_46_0: u32 = 16998;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: bool = {
            let value = state.read_register::<bool>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: cast zx s_46_1 -> bv
        let s_46_2: Bits = Bits::new(s_46_1 as u128, 1u16);
        // C s_46_3: const #1u : u8
        let s_46_3: bool = true;
        // C s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 1u16);
        // D s_46_5: cmp-eq s_46_2 s_46_4
        let s_46_5: bool = ((s_46_2) == (s_46_4));
        // D s_46_6: write-var gs#5934 <= s_46_5
        fn_state.gs_5934 = s_46_5;
        // N s_46_7: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1s : i
        let s_47_0: i128 = 1;
        // D s_47_1: read-var PL:i64
        let s_47_1: i64 = fn_state.PL;
        // D s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (i128::try_from(s_47_1).unwrap());
        // D s_47_3: sub s_47_2 s_47_0
        let s_47_3: i128 = ((s_47_2) - (s_47_0));
        // D s_47_4: cast reint s_47_3 -> i64
        let s_47_4: i64 = (s_47_3 as i64);
        // C s_47_5: const #0s : i
        let s_47_5: i128 = 0;
        // C s_47_6: const #816u : u32
        let s_47_6: u32 = 816;
        // D s_47_7: read-reg s_47_6:i64
        let s_47_7: i64 = {
            let value = state.read_register::<i64>(s_47_6 as isize);
            tracer.read_register(s_47_6 as isize, value);
            value
        };
        // D s_47_8: cast zx s_47_7 -> i
        let s_47_8: i128 = (i128::try_from(s_47_7).unwrap());
        // C s_47_9: const #14552u : u32
        let s_47_9: u32 = 14552;
        // D s_47_10: read-reg s_47_9:u256
        let s_47_10: u64 = {
            let value = state.read_register::<u64>(s_47_9 as isize);
            tracer.read_register(s_47_9 as isize, value);
            value
        };
        // D s_47_11: cast zx s_47_10 -> bv
        let s_47_11: Bits = Bits::new(s_47_10 as u128, 256u16);
        // D s_47_12: cast zx s_47_4 -> i
        let s_47_12: i128 = (i128::try_from(s_47_4).unwrap());
        // D s_47_13: call zext_subrange(s_47_8, s_47_11, s_47_12, s_47_5)
        let s_47_13: Bits = zext_subrange(
            state,
            tracer,
            s_47_8,
            s_47_11,
            s_47_12,
            s_47_5,
        );
        // D s_47_14: cast reint s_47_13 -> u256
        let s_47_14: u64 = (s_47_13.value() as u64);
        // C s_47_15: const #14552u : u32
        let s_47_15: u32 = 14552;
        // N s_47_16: write-reg s_47_15 <= s_47_14
        let s_47_16: () = {
            state.write_register::<u64>(s_47_15 as isize, s_47_14);
            tracer.write_register(s_47_15 as isize, s_47_14);
        };
        // N s_47_17: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var VL:i64
        let s_48_0: i64 = fn_state.VL;
        // D s_48_1: write-var ga#3918 <= s_48_0
        fn_state.ga_3918 = s_48_0;
        // N s_48_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #440u : u32
        let s_49_0: u32 = 440;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // D s_49_4: write-var gs#5974 <= s_49_3
        fn_state.gs_5974 = s_49_3;
        // N s_49_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #432u : u32
        let s_50_0: u32 = 432;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: assert s_50_3
        let s_50_4: () = assert!(s_50_3);
        // C s_50_5: const #102552u : u32
        let s_50_5: u32 = 102552;
        // D s_50_6: read-reg s_50_5:struct
        let s_50_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_5 as isize);
            tracer.read_register(s_50_5 as isize, value);
            value
        };
        // D s_50_7: call _get_HCR_EL2_Type_TGE(s_50_6)
        let s_50_7: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_50_6);
        // D s_50_8: cast zx s_50_7 -> bv
        let s_50_8: Bits = Bits::new(s_50_7 as u128, 1u16);
        // C s_50_9: const #0u : u8
        let s_50_9: bool = false;
        // C s_50_10: cast zx s_50_9 -> bv
        let s_50_10: Bits = Bits::new(s_50_9 as u128, 1u16);
        // D s_50_11: cmp-eq s_50_8 s_50_10
        let s_50_11: bool = ((s_50_8) == (s_50_10));
        // N s_50_12: branch s_50_11 b52 b51
        if s_50_11 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #448u : u32
        let s_51_0: u32 = 448;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call IsFPEnabled(s_51_1)
        let s_51_2: bool = IsFPEnabled(state, tracer, s_51_1);
        // D s_51_3: write-var lower_enabled <= s_51_2
        fn_state.lower_enabled = s_51_2;
        // N s_51_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #440u : u32
        let s_52_0: u32 = 440;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call IsFPEnabled(s_52_1)
        let s_52_2: bool = IsFPEnabled(state, tracer, s_52_1);
        // D s_52_3: write-var lower_enabled <= s_52_2
        fn_state.lower_enabled = s_52_2;
        // N s_52_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b55 b54
        if s_53_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #440u : u32
        let s_54_0: u32 = 440;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call IsFPEnabled(s_54_1)
        let s_54_2: bool = IsFPEnabled(state, tracer, s_54_1);
        // D s_54_3: write-var lower_enabled <= s_54_2
        fn_state.lower_enabled = s_54_2;
        // N s_54_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #432u : u32
        let s_55_0: u32 = 432;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call IsFPEnabled(s_55_1)
        let s_55_2: bool = IsFPEnabled(state, tracer, s_55_1);
        // D s_55_3: write-var lower_enabled <= s_55_2
        fn_state.lower_enabled = s_55_2;
        // N s_55_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#5908 <= s_57_0
        fn_state.gs_5908 = s_57_0;
        // N s_57_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
