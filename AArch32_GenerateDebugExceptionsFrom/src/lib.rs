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
use u_get_SDCR_Type_SPD::*;
use u_get_MDCR_EL3_Type_SDD::*;
use AArch64_GenerateDebugExceptionsFrom::*;
use ELUsingAArch32::*;
use Halted::*;
use u_get_MDCR_EL3_Type_SPD32::*;
use u_get_DBGOSLSR_Type_OSLK::*;
use IsSecureEL2Enabled::*;
use DBGOSLSR_read::*;
use DebugTargetFrom::*;
use DoubleLockStatus::*;
use AArch32_SelfHostedSecurePrivilegedInvasiveDebugEnabled::*;
use SDER_read::*;
use u_get_SDER_Type_SUIDEN::*;
use common::*;
pub fn AArch32_GenerateDebugExceptionsFrom<T: Tracer>(
    state: &mut State,
    tracer: &T,
    from_el: u8,
    from_state: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_5087: bool,
        return_value: bool,
        gs_5094: bool,
        enabled: bool,
        gs_5088: bool,
        spdshadow_61: u8,
        gs_5091: bool,
        from_el: u8,
        from_state: u32,
    }
    let fn_state = FunctionState {
        from_el,
        from_state,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var from_state:u32
        let s_0_0: u32 = fn_state.from_state;
        // D s_0_1: call DebugTargetFrom(s_0_0)
        let s_0_1: u8 = DebugTargetFrom(state, tracer, s_0_0);
        // D s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b31 b1
        if s_0_3 {
            return block_31(state, tracer, fn_state);
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
        // S s_1_1: call DBGOSLSR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_1_0);
        // S s_1_2: call _get_DBGOSLSR_Type_OSLK(s_1_1)
        let s_1_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // N s_1_7: branch s_1_6 b30 b2
        if s_1_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call DoubleLockStatus(s_2_0)
        let s_2_1: bool = DoubleLockStatus(state, tracer, s_2_0);
        // D s_2_2: write-var gs#5087 <= s_2_1
        fn_state.gs_5087 = s_2_1;
        // N s_2_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#5087:u8
        let s_3_0: bool = fn_state.gs_5087;
        // N s_3_1: branch s_3_0 b29 b4
        if s_3_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call Halted(s_4_0)
        let s_4_1: bool = Halted(state, tracer, s_4_0);
        // D s_4_2: write-var gs#5088 <= s_4_1
        fn_state.gs_5088 = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#5088:u8
        let s_5_0: bool = fn_state.gs_5088;
        // N s_5_1: branch s_5_0 b28 b6
        if s_5_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b27 b7
        if s_6_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#5091 <= s_7_0
        fn_state.gs_5091 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#5091:u8
        let s_8_0: bool = fn_state.gs_5091;
        // N s_8_1: branch s_8_0 b12 b9
        if s_8_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var from_el:u8
        let s_9_0: u8 = fn_state.from_el;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // D s_9_5: cmp-ne s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) != (s_9_4));
        // D s_9_6: write-var enabled <= s_9_5
        fn_state.enabled = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var enabled:u8
        let s_10_0: bool = fn_state.enabled;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var return_value:u8
        let s_11_0: bool = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var from_el:u8
        let s_12_0: u8 = fn_state.from_el;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #432u : u32
        let s_12_2: u32 = 432;
        // D s_12_3: read-reg s_12_2:u8
        let s_12_3: u8 = {
            let value = state.read_register::<u8>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 2u16);
        // D s_12_5: cmp-ne s_12_1 s_12_4
        let s_12_5: bool = ((s_12_1) != (s_12_4));
        // N s_12_6: assert s_12_5
        let s_12_6: () = assert!(s_12_5);
        // C s_12_7: const #() : ()
        let s_12_7: () = ();
        // S s_12_8: call IsSecureEL2Enabled(s_12_7)
        let s_12_8: bool = IsSecureEL2Enabled(state, tracer, s_12_7);
        // N s_12_9: branch s_12_8 b26 b13
        if s_12_8 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // N s_13_3: branch s_13_2 b25 b14
        if s_13_2 {
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
        // C s_14_0: const #22712u : u32
        let s_14_0: u32 = 22712;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_MDCR_EL3_Type_SPD32(s_14_1)
        let s_14_2: u8 = u_get_MDCR_EL3_Type_SPD32(state, tracer, s_14_1);
        // D s_14_3: write-var spdshadow#61 <= s_14_2
        fn_state.spdshadow_61 = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var spdshadow#61:u8
        let s_15_1: u8 = fn_state.spdshadow_61;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 2u16);
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
        // N s_15_22: branch s_15_21 b24 b16
        if s_15_21 {
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
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call AArch32_SelfHostedSecurePrivilegedInvasiveDebugEnabled(s_16_0)
        let s_16_1: bool = AArch32_SelfHostedSecurePrivilegedInvasiveDebugEnabled(
            state,
            tracer,
            s_16_0,
        );
        // D s_16_2: write-var enabled <= s_16_1
        fn_state.enabled = s_16_1;
        // N s_16_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var from_el:u8
        let s_17_0: u8 = fn_state.from_el;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #448u : u32
        let s_17_2: u32 = 448;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // N s_17_6: branch s_17_5 b20 b18
        if s_17_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var enabled:u8
        let s_20_0: bool = fn_state.enabled;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call SDER_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = SDER_read(state, tracer, s_21_0);
        // S s_21_2: call _get_SDER_Type_SUIDEN(s_21_1)
        let s_21_2: bool = u_get_SDER_Type_SUIDEN(state, tracer, s_21_1);
        // S s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // S s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#5094 <= s_21_6
        fn_state.gs_5094 = s_21_6;
        // N s_21_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#5094:u8
        let s_22_0: bool = fn_state.gs_5094;
        // D s_22_1: write-var enabled <= s_22_0
        fn_state.enabled = s_22_0;
        // N s_22_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#5094 <= s_23_0
        fn_state.gs_5094 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var spdshadow#61:u8
        let s_24_1: u8 = fn_state.spdshadow_61;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 2u16);
        // C s_24_3: const #1u : u64
        let s_24_3: u64 = 1;
        // D s_24_4: bit-extract s_24_2 s_24_0 s_24_3
        let s_24_4: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_5: cast reint s_24_4 -> u8
        let s_24_5: bool = ((s_24_4.value()) != 0);
        // C s_24_6: const #0s : i
        let s_24_6: i128 = 0;
        // C s_24_7: const #0u : u64
        let s_24_7: u64 = 0;
        // D s_24_8: cast zx s_24_5 -> u64
        let s_24_8: u64 = (s_24_5 as u64);
        // C s_24_9: const #1u : u64
        let s_24_9: u64 = 1;
        // D s_24_10: and s_24_8 s_24_9
        let s_24_10: u64 = ((s_24_8) & (s_24_9));
        // D s_24_11: cmp-eq s_24_10 s_24_9
        let s_24_11: bool = ((s_24_10) == (s_24_9));
        // D s_24_12: lsl s_24_8 s_24_6
        let s_24_12: u64 = s_24_8 << s_24_6;
        // D s_24_13: or s_24_7 s_24_12
        let s_24_13: u64 = ((s_24_7) | (s_24_12));
        // D s_24_14: cmpl s_24_12
        let s_24_14: u64 = !s_24_12;
        // D s_24_15: and s_24_7 s_24_14
        let s_24_15: u64 = ((s_24_7) & (s_24_14));
        // D s_24_16: select s_24_11 s_24_13 s_24_15
        let s_24_16: u64 = if s_24_11 { s_24_13 } else { s_24_15 };
        // D s_24_17: cast trunc s_24_16 -> u8
        let s_24_17: bool = ((s_24_16) != 0);
        // D s_24_18: cast zx s_24_17 -> bv
        let s_24_18: Bits = Bits::new(s_24_17 as u128, 1u16);
        // C s_24_19: const #1u : u8
        let s_24_19: bool = true;
        // C s_24_20: cast zx s_24_19 -> bv
        let s_24_20: Bits = Bits::new(s_24_19 as u128, 1u16);
        // D s_24_21: cmp-eq s_24_18 s_24_20
        let s_24_21: bool = ((s_24_18) == (s_24_20));
        // D s_24_22: write-var enabled <= s_24_21
        fn_state.enabled = s_24_21;
        // N s_24_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #15048u : u32
        let s_25_0: u32 = 15048;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_SDCR_Type_SPD(s_25_1)
        let s_25_2: u8 = u_get_SDCR_Type_SPD(state, tracer, s_25_1);
        // D s_25_3: write-var spdshadow#61 <= s_25_2
        fn_state.spdshadow_61 = s_25_2;
        // N s_25_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #22712u : u32
        let s_26_0: u32 = 22712;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_MDCR_EL3_Type_SDD(s_26_1)
        let s_26_2: bool = u_get_MDCR_EL3_Type_SDD(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #0u : u8
        let s_26_4: bool = false;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var enabled <= s_26_6
        fn_state.enabled = s_26_6;
        // N s_26_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var from_state:u32
        let s_27_0: u32 = fn_state.from_state;
        // C s_27_1: const #3u : u32
        let s_27_1: u32 = 3;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: write-var gs#5091 <= s_27_2
        fn_state.gs_5091 = s_27_2;
        // N s_27_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#5088 <= s_29_0
        fn_state.gs_5088 = s_29_0;
        // N s_29_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#5087 <= s_30_0
        fn_state.gs_5087 = s_30_0;
        // N s_30_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: read-var from_el:u8
        let s_31_1: u8 = fn_state.from_el;
        // D s_31_2: read-var from_state:u32
        let s_31_2: u32 = fn_state.from_state;
        // D s_31_3: call AArch64_GenerateDebugExceptionsFrom(s_31_1, s_31_2, s_31_0)
        let s_31_3: bool = AArch64_GenerateDebugExceptionsFrom(
            state,
            tracer,
            s_31_1,
            s_31_2,
            s_31_0,
        );
        // D s_31_4: write-var return_value <= s_31_3
        fn_state.return_value = s_31_3;
        // N s_31_5: jump b11
        return block_11(state, tracer, fn_state);
    }
}
