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
use u_update_CPACR_Type_cp11::*;
use Mk_CPACR_Type::*;
use CPACR_write::*;
use ELUsingAArch32::*;
use u_get_CPACR_Type_cp10::*;
use u_get_NSACR_Type_cp10::*;
use IsCurrentSecurityState::*;
use u_update_CPACR_Type_cp10::*;
use CPACR_read__1::*;
use u_get_CPACR_Type_cp11::*;
use common::*;
pub fn u__set_CPACR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_36353: bool,
        r: ProductType700c18a878c5601b,
        ga_28034: ProductType700c18a878c5601b,
        gs_36351: bool,
        gs_36354: bool,
        gs_36356: bool,
        ga_28040: ProductType700c18a878c5601b,
        gs_36352: bool,
        gs_36355: bool,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CPACR_read__1(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_0_2);
        // D s_0_4: write-var ga#28034 <= s_0_3
        fn_state.ga_28034 = s_0_3;
        // D s_0_5: read-var ga#28034.0:struct
        let s_0_5: u32 = fn_state.ga_28034._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #20s : i
        let s_0_8: i128 = 20;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u20
        let s_0_11: u32 = (s_0_10.value() as u32);
        // C s_0_12: const #20s : i
        let s_0_12: i128 = 20;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 20u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: call Mk_CPACR_Type(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = Mk_CPACR_Type(state, tracer, s_0_26);
        // D s_0_28: call CPACR_write(s_0_27)
        let s_0_28: () = CPACR_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call CPACR_read__1(s_0_29)
        let s_0_30: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_0_29);
        // D s_0_31: write-var ga#28040 <= s_0_30
        fn_state.ga_28040 = s_0_30;
        // D s_0_32: read-var ga#28040.0:struct
        let s_0_32: u32 = fn_state.ga_28040._0;
        // D s_0_33: read-var r.0:struct
        let s_0_33: u32 = fn_state.r._0;
        // C s_0_34: const #24s : i
        let s_0_34: i128 = 24;
        // C s_0_35: const #8s : i
        let s_0_35: i128 = 8;
        // D s_0_36: cast zx s_0_33 -> bv
        let s_0_36: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_37: bit-extract s_0_36 s_0_34 s_0_35
        let s_0_37: Bits = (Bits::new(
            ((s_0_36) >> (s_0_34)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u8
        let s_0_38: u8 = (s_0_37.value() as u8);
        // C s_0_39: const #8s : i
        let s_0_39: i128 = 8;
        // C s_0_40: const #24s : i
        let s_0_40: i128 = 24;
        // D s_0_41: cast zx s_0_32 -> bv
        let s_0_41: Bits = Bits::new(s_0_32 as u128, 32u16);
        // D s_0_42: cast zx s_0_38 -> bv
        let s_0_42: Bits = Bits::new(s_0_38 as u128, 8u16);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_39
        let s_0_45: Bits = s_0_44 << s_0_39;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_42 s_0_46
        let s_0_47: Bits = ((s_0_42) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_40
        let s_0_48: Bits = s_0_47 << s_0_40;
        // C s_0_49: lsl s_0_46 s_0_40
        let s_0_49: Bits = s_0_46 << s_0_40;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_41 s_0_50
        let s_0_51: Bits = ((s_0_41) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u32
        let s_0_53: u32 = (s_0_52.value() as u32);
        // D s_0_54: call Mk_CPACR_Type(s_0_53)
        let s_0_54: ProductType700c18a878c5601b = Mk_CPACR_Type(state, tracer, s_0_53);
        // D s_0_55: call CPACR_write(s_0_54)
        let s_0_55: () = CPACR_write(state, tracer, s_0_54);
        // C s_0_56: const #424u : u32
        let s_0_56: u32 = 424;
        // D s_0_57: read-reg s_0_56:u8
        let s_0_57: u8 = {
            let value = state.read_register::<u8>(s_0_56 as isize);
            tracer.read_register(s_0_56 as isize, value);
            value
        };
        // C s_0_58: const #2u : u8
        let s_0_58: u8 = 2;
        // D s_0_59: cmp-lt s_0_57 s_0_58
        let s_0_59: bool = ((s_0_57) < (s_0_58));
        // N s_0_60: branch s_0_59 b23 b1
        if s_0_59 {
            return block_23(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#36351 <= s_1_0
        fn_state.gs_36351 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#36351:u8
        let s_2_0: bool = fn_state.gs_36351;
        // N s_2_1: branch s_2_0 b22 b3
        if s_2_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#36352 <= s_3_0
        fn_state.gs_36352 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#36352:u8
        let s_4_0: bool = fn_state.gs_36352;
        // N s_4_1: branch s_4_0 b21 b5
        if s_4_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#36353 <= s_5_0
        fn_state.gs_36353 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#36353:u8
        let s_6_0: bool = fn_state.gs_36353;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b20 b7
        if s_6_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #424u : u32
        let s_8_0: u32 = 424;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b19 b9
        if s_8_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#36354 <= s_9_0
        fn_state.gs_36354 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#36354:u8
        let s_10_0: bool = fn_state.gs_36354;
        // N s_10_1: branch s_10_0 b18 b11
        if s_10_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#36355 <= s_11_0
        fn_state.gs_36355 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#36355:u8
        let s_12_0: bool = fn_state.gs_36355;
        // N s_12_1: branch s_12_0 b17 b13
        if s_12_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#36356 <= s_13_0
        fn_state.gs_36356 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#36356:u8
        let s_14_0: bool = fn_state.gs_36356;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
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
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call CPACR_read__1(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_16_0);
        // D s_16_2: read-var r:struct
        let s_16_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_16_3: call _get_CPACR_Type_cp10(s_16_2)
        let s_16_3: u8 = u_get_CPACR_Type_cp10(state, tracer, s_16_2);
        // D s_16_4: call _update_CPACR_Type_cp10(s_16_1, s_16_3)
        let s_16_4: ProductType700c18a878c5601b = u_update_CPACR_Type_cp10(
            state,
            tracer,
            s_16_1,
            s_16_3,
        );
        // D s_16_5: call CPACR_write(s_16_4)
        let s_16_5: () = CPACR_write(state, tracer, s_16_4);
        // N s_16_6: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #102488u : u32
        let s_17_0: u32 = 102488;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_NSACR_Type_cp10(s_17_1)
        let s_17_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #0u : u8
        let s_17_4: bool = false;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // D s_17_7: write-var gs#36356 <= s_17_6
        fn_state.gs_36356 = s_17_6;
        // N s_17_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #3u : u32
        let s_18_0: u32 = 3;
        // S s_18_1: call IsCurrentSecurityState(s_18_0)
        let s_18_1: bool = IsCurrentSecurityState(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // D s_18_3: write-var gs#36355 <= s_18_2
        fn_state.gs_36355 = s_18_2;
        // N s_18_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call ELUsingAArch32(s_19_1)
        let s_19_2: bool = ELUsingAArch32(state, tracer, s_19_1);
        // D s_19_3: write-var gs#36354 <= s_19_2
        fn_state.gs_36354 = s_19_2;
        // N s_19_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call CPACR_read__1(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_20_0);
        // D s_20_2: read-var r:struct
        let s_20_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_20_3: call _get_CPACR_Type_cp11(s_20_2)
        let s_20_3: u8 = u_get_CPACR_Type_cp11(state, tracer, s_20_2);
        // D s_20_4: call _update_CPACR_Type_cp11(s_20_1, s_20_3)
        let s_20_4: ProductType700c18a878c5601b = u_update_CPACR_Type_cp11(
            state,
            tracer,
            s_20_1,
            s_20_3,
        );
        // D s_20_5: call CPACR_write(s_20_4)
        let s_20_5: () = CPACR_write(state, tracer, s_20_4);
        // N s_20_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #102488u : u32
        let s_21_0: u32 = 102488;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_NSACR_Type_cp10(s_21_1)
        let s_21_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #0u : u8
        let s_21_4: bool = false;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#36353 <= s_21_6
        fn_state.gs_36353 = s_21_6;
        // N s_21_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #3u : u32
        let s_22_0: u32 = 3;
        // S s_22_1: call IsCurrentSecurityState(s_22_0)
        let s_22_1: bool = IsCurrentSecurityState(state, tracer, s_22_0);
        // S s_22_2: not s_22_1
        let s_22_2: bool = !s_22_1;
        // D s_22_3: write-var gs#36352 <= s_22_2
        fn_state.gs_36352 = s_22_2;
        // N s_22_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call ELUsingAArch32(s_23_1)
        let s_23_2: bool = ELUsingAArch32(state, tracer, s_23_1);
        // D s_23_3: write-var gs#36351 <= s_23_2
        fn_state.gs_36351 = s_23_2;
        // N s_23_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
