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
use u_get_TCR_EL2_Type_MTX::*;
use u_get_TCR_EL3_Type_MTX::*;
use HaveMTE4Ext::*;
use u_get_TCR_EL2_Type_MTX1::*;
use ELUsingAArch32::*;
use S1TranslationRegime::*;
use u_get_TCR_EL2_Type_MTX0::*;
use u_get_TCR_EL1_Type_MTX1::*;
use ELIsInHost::*;
use u_get_TCR_EL1_Type_MTX0::*;
use HaveVirtHostExt::*;
use common::*;
pub fn EffectiveMTX<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    is_instr: bool,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        mtx: bool,
        regime: u8,
        gs_6829: bool,
        gs_6823: bool,
        address: u64,
        is_instr: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        address,
        is_instr,
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var el:u8
        let s_0_4: u8 = fn_state.el;
        // D s_0_5: call S1TranslationRegime(s_0_4)
        let s_0_5: u8 = S1TranslationRegime(state, tracer, s_0_4);
        // D s_0_6: write-var regime <= s_0_5
        fn_state.regime = s_0_5;
        // D s_0_7: read-var regime:u8
        let s_0_7: u8 = fn_state.regime;
        // D s_0_8: call ELUsingAArch32(s_0_7)
        let s_0_8: bool = ELUsingAArch32(state, tracer, s_0_7);
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: assert s_0_9
        let s_0_10: () = assert!(s_0_9);
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call HaveMTE4Ext(s_0_11)
        let s_0_12: bool = HaveMTE4Ext(state, tracer, s_0_11);
        // S s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b24 b1
        if s_0_13 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var is_instr:u8
        let s_1_0: bool = fn_state.is_instr;
        // D s_1_1: write-var gs#6823 <= s_1_0
        fn_state.gs_6823 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6823:u8
        let s_2_0: bool = fn_state.gs_6823;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var regime:u8
        let s_3_0: u8 = fn_state.regime;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
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
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b10 b4
        if s_3_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #55s : i
        let s_4_0: i128 = 55;
        // D s_4_1: read-var address:u64
        let s_4_1: u64 = fn_state.address;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b9 b5
        if s_4_21 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #22392u : u32
        let s_5_0: u32 = 22392;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_TCR_EL1_Type_MTX0(s_5_1)
        let s_5_2: bool = u_get_TCR_EL1_Type_MTX0(state, tracer, s_5_1);
        // D s_5_3: write-var mtx <= s_5_2
        fn_state.mtx = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var mtx:u8
        let s_8_0: bool = fn_state.mtx;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #22392u : u32
        let s_9_0: u32 = 22392;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_TCR_EL1_Type_MTX1(s_9_1)
        let s_9_2: bool = u_get_TCR_EL1_Type_MTX1(state, tracer, s_9_1);
        // D s_9_3: write-var mtx <= s_9_2
        fn_state.mtx = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var regime:u8
        let s_10_0: u8 = fn_state.regime;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #432u : u32
        let s_10_2: u32 = 432;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // D s_10_5: cmp-eq s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) == (s_10_4));
        // D s_10_6: not s_10_5
        let s_10_6: bool = !s_10_5;
        // N s_10_7: branch s_10_6 b20 b11
        if s_10_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveVirtHostExt(s_11_0)
        let s_11_1: bool = HaveVirtHostExt(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b19 b12
        if s_11_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#6829 <= s_12_0
        fn_state.gs_6829 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#6829:u8
        let s_13_0: bool = fn_state.gs_6829;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #12816u : u32
        let s_14_0: u32 = 12816;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_TCR_EL2_Type_MTX(s_14_1)
        let s_14_2: bool = u_get_TCR_EL2_Type_MTX(state, tracer, s_14_1);
        // D s_14_3: write-var mtx <= s_14_2
        fn_state.mtx = s_14_2;
        // N s_14_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #55s : i
        let s_15_0: i128 = 55;
        // D s_15_1: read-var address:u64
        let s_15_1: u64 = fn_state.address;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 64u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // N s_15_22: branch s_15_21 b18 b16
        if s_15_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #12816u : u32
        let s_16_0: u32 = 12816;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_TCR_EL2_Type_MTX0(s_16_1)
        let s_16_2: bool = u_get_TCR_EL2_Type_MTX0(state, tracer, s_16_1);
        // D s_16_3: write-var mtx <= s_16_2
        fn_state.mtx = s_16_2;
        // N s_16_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_17_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #12816u : u32
        let s_18_0: u32 = 12816;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_TCR_EL2_Type_MTX1(s_18_1)
        let s_18_2: bool = u_get_TCR_EL2_Type_MTX1(state, tracer, s_18_1);
        // D s_18_3: write-var mtx <= s_18_2
        fn_state.mtx = s_18_2;
        // N s_18_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var el:u8
        let s_19_0: u8 = fn_state.el;
        // D s_19_1: call ELIsInHost(s_19_0)
        let s_19_1: bool = ELIsInHost(state, tracer, s_19_0);
        // D s_19_2: write-var gs#6829 <= s_19_1
        fn_state.gs_6829 = s_19_1;
        // N s_19_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var regime:u8
        let s_20_0: u8 = fn_state.regime;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #424u : u32
        let s_20_2: u32 = 424;
        // D s_20_3: read-reg s_20_2:u8
        let s_20_3: u8 = {
            let value = state.read_register::<u8>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 2u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // D s_20_6: not s_20_5
        let s_20_6: bool = !s_20_5;
        // N s_20_7: branch s_20_6 b22 b21
        if s_20_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #10736u : u32
        let s_21_0: u32 = 10736;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_TCR_EL3_Type_MTX(s_21_1)
        let s_21_2: bool = u_get_TCR_EL3_Type_MTX(state, tracer, s_21_1);
        // D s_21_3: write-var mtx <= s_21_2
        fn_state.mtx = s_21_2;
        // N s_21_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_22_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var mtx <= s_23_0
        fn_state.mtx = s_23_0;
        // N s_23_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#6823 <= s_24_0
        fn_state.gs_6823 = s_24_0;
        // N s_24_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
