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
use u_get_MDSCR_EL1_Type_HDE::*;
use u_get_MDSCR_EL1_Type_SC2::*;
use u_get_MDSCR_EL1_Type_RXO::*;
use u_get_MDSCR_EL1_Type_TXfull::*;
use u_get_MDSCR_EL1_Type_RXfull::*;
use u_get_MDSCR_EL1_Type_TFO::*;
use u_get_MDSCR_EL1_Type_INTdis::*;
use u_get_OSLSR_EL1_Type_OSLK::*;
use Bit::*;
use u_get_MDSCR_EL1_Type_ERR::*;
use u_get_MDSCR_EL1_Type_TDA::*;
use u_get_MDSCR_EL1_Type_EHBWE::*;
use u_get_MDSCR_EL1_Type_TXU::*;
use common::*;
pub fn u__set_MDSCR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
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
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #104648u : u32
        let s_0_2: u32 = 104648;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #104648u : u32
        let s_0_4: u32 = 104648;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #104648u : u32
        let s_0_6: u32 = 104648;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #104648u : u32
        let s_0_8: u32 = 104648;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #104648u : u32
        let s_0_10: u32 = 104648;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #104648u : u32
        let s_0_12: u32 = 104648;
        // N s_0_13: write-reg s_0_12 <= s_0_11
        let s_0_13: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize, s_0_11);
            tracer.write_register(s_0_12 as isize, s_0_11);
        };
        // D s_0_14: read-var r.0:struct
        let s_0_14: u64 = fn_state.r._0;
        // C s_0_15: const #20s : i
        let s_0_15: i128 = 20;
        // C s_0_16: const #1s : i
        let s_0_16: i128 = 1;
        // D s_0_17: cast zx s_0_14 -> bv
        let s_0_17: Bits = Bits::new(s_0_14 as u128, 64u16);
        // D s_0_18: bit-extract s_0_17 s_0_15 s_0_16
        let s_0_18: Bits = (Bits::new(
            ((s_0_17) >> (s_0_15)).value(),
            u16::try_from(s_0_16).unwrap(),
        ));
        // D s_0_19: cast reint s_0_18 -> u8
        let s_0_19: bool = ((s_0_18.value()) != 0);
        // D s_0_20: call Bit(s_0_19)
        let s_0_20: bool = Bit(state, tracer, s_0_19);
        // C s_0_21: const #104648u : u32
        let s_0_21: u32 = 104648;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // C s_0_23: const #104648u : u32
        let s_0_23: u32 = 104648;
        // N s_0_24: write-reg s_0_23 <= s_0_22
        let s_0_24: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize, s_0_22);
            tracer.write_register(s_0_23 as isize, s_0_22);
        };
        // C s_0_25: const #104648u : u32
        let s_0_25: u32 = 104648;
        // D s_0_26: read-reg s_0_25:struct
        let s_0_26: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // C s_0_27: const #104648u : u32
        let s_0_27: u32 = 104648;
        // N s_0_28: write-reg s_0_27 <= s_0_26
        let s_0_28: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize, s_0_26);
            tracer.write_register(s_0_27 as isize, s_0_26);
        };
        // D s_0_29: read-var r.0:struct
        let s_0_29: u64 = fn_state.r._0;
        // C s_0_30: const #28s : i
        let s_0_30: i128 = 28;
        // C s_0_31: const #1s : i
        let s_0_31: i128 = 1;
        // D s_0_32: cast zx s_0_29 -> bv
        let s_0_32: Bits = Bits::new(s_0_29 as u128, 64u16);
        // D s_0_33: bit-extract s_0_32 s_0_30 s_0_31
        let s_0_33: Bits = (Bits::new(
            ((s_0_32) >> (s_0_30)).value(),
            u16::try_from(s_0_31).unwrap(),
        ));
        // D s_0_34: cast reint s_0_33 -> u8
        let s_0_34: bool = ((s_0_33.value()) != 0);
        // D s_0_35: call Bit(s_0_34)
        let s_0_35: bool = Bit(state, tracer, s_0_34);
        // C s_0_36: const #104648u : u32
        let s_0_36: u32 = 104648;
        // D s_0_37: read-reg s_0_36:struct
        let s_0_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_36 as isize);
            tracer.read_register(s_0_36 as isize, value);
            value
        };
        // C s_0_38: const #104648u : u32
        let s_0_38: u32 = 104648;
        // N s_0_39: write-reg s_0_38 <= s_0_37
        let s_0_39: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_38 as isize, s_0_37);
            tracer.write_register(s_0_38 as isize, s_0_37);
        };
        // C s_0_40: const #104648u : u32
        let s_0_40: u32 = 104648;
        // D s_0_41: read-reg s_0_40:struct
        let s_0_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_40 as isize);
            tracer.read_register(s_0_40 as isize, value);
            value
        };
        // C s_0_42: const #104648u : u32
        let s_0_42: u32 = 104648;
        // N s_0_43: write-reg s_0_42 <= s_0_41
        let s_0_43: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_42 as isize, s_0_41);
            tracer.write_register(s_0_42 as isize, s_0_41);
        };
        // C s_0_44: const #104648u : u32
        let s_0_44: u32 = 104648;
        // D s_0_45: read-reg s_0_44:struct
        let s_0_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_44 as isize);
            tracer.read_register(s_0_44 as isize, value);
            value
        };
        // C s_0_46: const #104648u : u32
        let s_0_46: u32 = 104648;
        // N s_0_47: write-reg s_0_46 <= s_0_45
        let s_0_47: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_46 as isize, s_0_45);
            tracer.write_register(s_0_46 as isize, s_0_45);
        };
        // C s_0_48: const #10128u : u32
        let s_0_48: u32 = 10128;
        // D s_0_49: read-reg s_0_48:struct
        let s_0_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_48 as isize);
            tracer.read_register(s_0_48 as isize, value);
            value
        };
        // D s_0_50: call _get_OSLSR_EL1_Type_OSLK(s_0_49)
        let s_0_50: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_0_49);
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 1u16);
        // C s_0_52: const #0u : u8
        let s_0_52: bool = false;
        // C s_0_53: cast zx s_0_52 -> bv
        let s_0_53: Bits = Bits::new(s_0_52 as u128, 1u16);
        // D s_0_54: cmp-eq s_0_51 s_0_53
        let s_0_54: bool = ((s_0_51) == (s_0_53));
        // D s_0_55: not s_0_54
        let s_0_55: bool = !s_0_54;
        // N s_0_56: branch s_0_55 b32 b1
        if s_0_55 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #10128u : u32
        let s_2_0: u32 = 10128;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_OSLSR_EL1_Type_OSLK(s_2_1)
        let s_2_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b31 b3
        if s_2_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #10128u : u32
        let s_4_0: u32 = 10128;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_OSLSR_EL1_Type_OSLK(s_4_1)
        let s_4_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b30 b5
        if s_4_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #10128u : u32
        let s_6_0: u32 = 10128;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_OSLSR_EL1_Type_OSLK(s_6_1)
        let s_6_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b29 b7
        if s_6_6 {
            return block_29(state, tracer, fn_state);
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
        // C s_8_0: const #10128u : u32
        let s_8_0: u32 = 10128;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_OSLSR_EL1_Type_OSLK(s_8_1)
        let s_8_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b28 b9
        if s_8_6 {
            return block_28(state, tracer, fn_state);
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
        // C s_10_0: const #10128u : u32
        let s_10_0: u32 = 10128;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_OSLSR_EL1_Type_OSLK(s_10_1)
        let s_10_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b27 b11
        if s_10_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #10128u : u32
        let s_12_0: u32 = 10128;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_OSLSR_EL1_Type_OSLK(s_12_1)
        let s_12_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b26 b13
        if s_12_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #10128u : u32
        let s_14_0: u32 = 10128;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_OSLSR_EL1_Type_OSLK(s_14_1)
        let s_14_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b25 b15
        if s_14_6 {
            return block_25(state, tracer, fn_state);
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
        // C s_16_0: const #10128u : u32
        let s_16_0: u32 = 10128;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_OSLSR_EL1_Type_OSLK(s_16_1)
        let s_16_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // N s_16_7: branch s_16_6 b24 b17
        if s_16_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #10128u : u32
        let s_18_0: u32 = 10128;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_OSLSR_EL1_Type_OSLK(s_18_1)
        let s_18_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b23 b19
        if s_18_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #10128u : u32
        let s_20_0: u32 = 10128;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_OSLSR_EL1_Type_OSLK(s_20_1)
        let s_20_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #1u : u8
        let s_20_4: bool = true;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
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
    ) -> () {
        // N s_21_0: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var r:struct
        let s_22_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_22_1: call _get_MDSCR_EL1_Type_ERR(s_22_0)
        let s_22_1: bool = u_get_MDSCR_EL1_Type_ERR(state, tracer, s_22_0);
        // C s_22_2: const #104648u : u32
        let s_22_2: u32 = 104648;
        // D s_22_3: read-reg s_22_2:struct
        let s_22_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // C s_22_4: const #104648u : u32
        let s_22_4: u32 = 104648;
        // N s_22_5: write-reg s_22_4 <= s_22_3
        let s_22_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_22_4 as isize, s_22_3);
            tracer.write_register(s_22_4 as isize, s_22_3);
        };
        // N s_22_6: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var r:struct
        let s_23_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_23_1: call _get_MDSCR_EL1_Type_HDE(s_23_0)
        let s_23_1: bool = u_get_MDSCR_EL1_Type_HDE(state, tracer, s_23_0);
        // C s_23_2: const #104648u : u32
        let s_23_2: u32 = 104648;
        // D s_23_3: read-reg s_23_2:struct
        let s_23_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_2 as isize);
            tracer.read_register(s_23_2 as isize, value);
            value
        };
        // C s_23_4: const #104648u : u32
        let s_23_4: u32 = 104648;
        // N s_23_5: write-reg s_23_4 <= s_23_3
        let s_23_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_23_4 as isize, s_23_3);
            tracer.write_register(s_23_4 as isize, s_23_3);
        };
        // N s_23_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var r:struct
        let s_24_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_24_1: call _get_MDSCR_EL1_Type_SC2(s_24_0)
        let s_24_1: bool = u_get_MDSCR_EL1_Type_SC2(state, tracer, s_24_0);
        // C s_24_2: const #104648u : u32
        let s_24_2: u32 = 104648;
        // D s_24_3: read-reg s_24_2:struct
        let s_24_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // C s_24_4: const #104648u : u32
        let s_24_4: u32 = 104648;
        // N s_24_5: write-reg s_24_4 <= s_24_3
        let s_24_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_24_4 as isize, s_24_3);
            tracer.write_register(s_24_4 as isize, s_24_3);
        };
        // N s_24_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var r:struct
        let s_25_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_25_1: call _get_MDSCR_EL1_Type_TDA(s_25_0)
        let s_25_1: bool = u_get_MDSCR_EL1_Type_TDA(state, tracer, s_25_0);
        // C s_25_2: const #104648u : u32
        let s_25_2: u32 = 104648;
        // D s_25_3: read-reg s_25_2:struct
        let s_25_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_2 as isize);
            tracer.read_register(s_25_2 as isize, value);
            value
        };
        // C s_25_4: const #104648u : u32
        let s_25_4: u32 = 104648;
        // N s_25_5: write-reg s_25_4 <= s_25_3
        let s_25_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_25_4 as isize, s_25_3);
            tracer.write_register(s_25_4 as isize, s_25_3);
        };
        // N s_25_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var r:struct
        let s_26_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_26_1: call _get_MDSCR_EL1_Type_INTdis(s_26_0)
        let s_26_1: u8 = u_get_MDSCR_EL1_Type_INTdis(state, tracer, s_26_0);
        // C s_26_2: const #104648u : u32
        let s_26_2: u32 = 104648;
        // D s_26_3: read-reg s_26_2:struct
        let s_26_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_2 as isize);
            tracer.read_register(s_26_2 as isize, value);
            value
        };
        // C s_26_4: const #104648u : u32
        let s_26_4: u32 = 104648;
        // N s_26_5: write-reg s_26_4 <= s_26_3
        let s_26_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_26_4 as isize, s_26_3);
            tracer.write_register(s_26_4 as isize, s_26_3);
        };
        // N s_26_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var r:struct
        let s_27_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_27_1: call _get_MDSCR_EL1_Type_TXU(s_27_0)
        let s_27_1: bool = u_get_MDSCR_EL1_Type_TXU(state, tracer, s_27_0);
        // C s_27_2: const #104648u : u32
        let s_27_2: u32 = 104648;
        // D s_27_3: read-reg s_27_2:struct
        let s_27_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // C s_27_4: const #104648u : u32
        let s_27_4: u32 = 104648;
        // N s_27_5: write-reg s_27_4 <= s_27_3
        let s_27_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_27_4 as isize, s_27_3);
            tracer.write_register(s_27_4 as isize, s_27_3);
        };
        // N s_27_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var r:struct
        let s_28_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_28_1: call _get_MDSCR_EL1_Type_RXO(s_28_0)
        let s_28_1: bool = u_get_MDSCR_EL1_Type_RXO(state, tracer, s_28_0);
        // C s_28_2: const #104648u : u32
        let s_28_2: u32 = 104648;
        // D s_28_3: read-reg s_28_2:struct
        let s_28_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // C s_28_4: const #104648u : u32
        let s_28_4: u32 = 104648;
        // N s_28_5: write-reg s_28_4 <= s_28_3
        let s_28_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_28_4 as isize, s_28_3);
            tracer.write_register(s_28_4 as isize, s_28_3);
        };
        // N s_28_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var r:struct
        let s_29_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_29_1: call _get_MDSCR_EL1_Type_TXfull(s_29_0)
        let s_29_1: bool = u_get_MDSCR_EL1_Type_TXfull(state, tracer, s_29_0);
        // C s_29_2: const #104648u : u32
        let s_29_2: u32 = 104648;
        // D s_29_3: read-reg s_29_2:struct
        let s_29_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // C s_29_4: const #104648u : u32
        let s_29_4: u32 = 104648;
        // N s_29_5: write-reg s_29_4 <= s_29_3
        let s_29_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_29_4 as isize, s_29_3);
            tracer.write_register(s_29_4 as isize, s_29_3);
        };
        // N s_29_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var r:struct
        let s_30_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_30_1: call _get_MDSCR_EL1_Type_RXfull(s_30_0)
        let s_30_1: bool = u_get_MDSCR_EL1_Type_RXfull(state, tracer, s_30_0);
        // C s_30_2: const #104648u : u32
        let s_30_2: u32 = 104648;
        // D s_30_3: read-reg s_30_2:struct
        let s_30_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // C s_30_4: const #104648u : u32
        let s_30_4: u32 = 104648;
        // N s_30_5: write-reg s_30_4 <= s_30_3
        let s_30_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_30_4 as isize, s_30_3);
            tracer.write_register(s_30_4 as isize, s_30_3);
        };
        // N s_30_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var r:struct
        let s_31_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_31_1: call _get_MDSCR_EL1_Type_TFO(s_31_0)
        let s_31_1: bool = u_get_MDSCR_EL1_Type_TFO(state, tracer, s_31_0);
        // C s_31_2: const #104648u : u32
        let s_31_2: u32 = 104648;
        // D s_31_3: read-reg s_31_2:struct
        let s_31_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_2 as isize);
            tracer.read_register(s_31_2 as isize, value);
            value
        };
        // C s_31_4: const #104648u : u32
        let s_31_4: u32 = 104648;
        // N s_31_5: write-reg s_31_4 <= s_31_3
        let s_31_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_31_4 as isize, s_31_3);
            tracer.write_register(s_31_4 as isize, s_31_3);
        };
        // N s_31_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var r:struct
        let s_32_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_32_1: call _get_MDSCR_EL1_Type_EHBWE(s_32_0)
        let s_32_1: bool = u_get_MDSCR_EL1_Type_EHBWE(state, tracer, s_32_0);
        // C s_32_2: const #104648u : u32
        let s_32_2: u32 = 104648;
        // D s_32_3: read-reg s_32_2:struct
        let s_32_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // C s_32_4: const #104648u : u32
        let s_32_4: u32 = 104648;
        // N s_32_5: write-reg s_32_4 <= s_32_3
        let s_32_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_32_4 as isize, s_32_3);
            tracer.write_register(s_32_4 as isize, s_32_3);
        };
        // N s_32_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
