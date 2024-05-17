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
use PMUEvent::*;
use Mk_BRBINFINJ_EL1_Type::*;
use u_get_BRBINFINJ_EL1_Type_MPRED::*;
use u_get_BRBINFINJ_EL1_Type_CCU::*;
use u_get_BRBINFINJ_EL1_Type_LASTFAILED::*;
use u_get_BRBINFINJ_EL1_Type_TYPE::*;
use u_get_BRBSRCINJ_EL1_Type_ADDRESS::*;
use ConstrainUnpredictableBool::*;
use u_get_BRBTGTINJ_EL1_Type_ADDRESS::*;
use u_get_BRBINFINJ_EL1_Type_EL::*;
use u_get_BRBINFINJ_EL1_Type_CC::*;
use u_get_BRBINFINJ_EL1_Type_T::*;
use u_get_BRBINFINJ_EL1_Type_VALID::*;
use u__UNKNOWN_bits::*;
use Mk_BRBTGTINJ_EL1_Type::*;
use UpdateBranchRecordBuffer::*;
use Mk_BRBSRCINJ_EL1_Type::*;
use common::*;
pub fn BRB_INJ<T: Tracer>(state: &mut State, tracer: &T, gs_26350: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26350: (),
    }
    let fn_state = FunctionState {
        gs_26350,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #10168u : u32
        let s_0_0: u32 = 10168;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_BRBINFINJ_EL1_Type_CCU(s_0_1)
        let s_0_2: bool = u_get_BRBINFINJ_EL1_Type_CCU(state, tracer, s_0_1);
        // C s_0_3: const #10168u : u32
        let s_0_3: u32 = 10168;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_BRBINFINJ_EL1_Type_CC(s_0_4)
        let s_0_5: u16 = u_get_BRBINFINJ_EL1_Type_CC(state, tracer, s_0_4);
        // C s_0_6: const #10168u : u32
        let s_0_6: u32 = 10168;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_BRBINFINJ_EL1_Type_LASTFAILED(s_0_7)
        let s_0_8: bool = u_get_BRBINFINJ_EL1_Type_LASTFAILED(state, tracer, s_0_7);
        // C s_0_9: const #10168u : u32
        let s_0_9: u32 = 10168;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_BRBINFINJ_EL1_Type_T(s_0_10)
        let s_0_11: bool = u_get_BRBINFINJ_EL1_Type_T(state, tracer, s_0_10);
        // C s_0_12: const #10168u : u32
        let s_0_12: u32 = 10168;
        // D s_0_13: read-reg s_0_12:struct
        let s_0_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: call _get_BRBINFINJ_EL1_Type_TYPE(s_0_13)
        let s_0_14: u8 = u_get_BRBINFINJ_EL1_Type_TYPE(state, tracer, s_0_13);
        // C s_0_15: const #10168u : u32
        let s_0_15: u32 = 10168;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_BRBINFINJ_EL1_Type_EL(s_0_16)
        let s_0_17: u8 = u_get_BRBINFINJ_EL1_Type_EL(state, tracer, s_0_16);
        // C s_0_18: const #10168u : u32
        let s_0_18: u32 = 10168;
        // D s_0_19: read-reg s_0_18:struct
        let s_0_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: call _get_BRBINFINJ_EL1_Type_MPRED(s_0_19)
        let s_0_20: bool = u_get_BRBINFINJ_EL1_Type_MPRED(state, tracer, s_0_19);
        // C s_0_21: const #10168u : u32
        let s_0_21: u32 = 10168;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: call _get_BRBINFINJ_EL1_Type_VALID(s_0_22)
        let s_0_23: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_0_22);
        // C s_0_24: const #21928u : u32
        let s_0_24: u32 = 21928;
        // D s_0_25: read-reg s_0_24:struct
        let s_0_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: call _get_BRBSRCINJ_EL1_Type_ADDRESS(s_0_25)
        let s_0_26: u64 = u_get_BRBSRCINJ_EL1_Type_ADDRESS(state, tracer, s_0_25);
        // C s_0_27: const #102304u : u32
        let s_0_27: u32 = 102304;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_BRBTGTINJ_EL1_Type_ADDRESS(s_0_28)
        let s_0_29: u64 = u_get_BRBTGTINJ_EL1_Type_ADDRESS(state, tracer, s_0_28);
        // D s_0_30: call UpdateBranchRecordBuffer(s_0_2, s_0_5, s_0_8, s_0_11, s_0_14, s_0_17, s_0_20, s_0_23, s_0_26, s_0_29)
        let s_0_30: () = UpdateBranchRecordBuffer(
            state,
            tracer,
            s_0_2,
            s_0_5,
            s_0_8,
            s_0_11,
            s_0_14,
            s_0_17,
            s_0_20,
            s_0_23,
            s_0_26,
            s_0_29,
        );
        // C s_0_31: const #64s : i64
        let s_0_31: i64 = 64;
        // C s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // S s_0_33: call __UNKNOWN_bits(s_0_32)
        let s_0_33: Bits = u__UNKNOWN_bits(state, tracer, s_0_32);
        // S s_0_34: cast reint s_0_33 -> u64
        let s_0_34: u64 = (s_0_33.value() as u64);
        // S s_0_35: call Mk_BRBINFINJ_EL1_Type(s_0_34)
        let s_0_35: ProductType5c790c8ef59cc8b2 = Mk_BRBINFINJ_EL1_Type(
            state,
            tracer,
            s_0_34,
        );
        // C s_0_36: const #10168u : u32
        let s_0_36: u32 = 10168;
        // N s_0_37: write-reg s_0_36 <= s_0_35
        let s_0_37: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_36 as isize, s_0_35);
            tracer.write_register(s_0_36 as isize, s_0_35);
        };
        // C s_0_38: const #64s : i64
        let s_0_38: i64 = 64;
        // C s_0_39: cast zx s_0_38 -> i
        let s_0_39: i128 = (i128::try_from(s_0_38).unwrap());
        // S s_0_40: call __UNKNOWN_bits(s_0_39)
        let s_0_40: Bits = u__UNKNOWN_bits(state, tracer, s_0_39);
        // S s_0_41: cast reint s_0_40 -> u64
        let s_0_41: u64 = (s_0_40.value() as u64);
        // S s_0_42: call Mk_BRBSRCINJ_EL1_Type(s_0_41)
        let s_0_42: ProductType5c790c8ef59cc8b2 = Mk_BRBSRCINJ_EL1_Type(
            state,
            tracer,
            s_0_41,
        );
        // C s_0_43: const #21928u : u32
        let s_0_43: u32 = 21928;
        // N s_0_44: write-reg s_0_43 <= s_0_42
        let s_0_44: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize, s_0_42);
            tracer.write_register(s_0_43 as isize, s_0_42);
        };
        // C s_0_45: const #64s : i64
        let s_0_45: i64 = 64;
        // C s_0_46: cast zx s_0_45 -> i
        let s_0_46: i128 = (i128::try_from(s_0_45).unwrap());
        // S s_0_47: call __UNKNOWN_bits(s_0_46)
        let s_0_47: Bits = u__UNKNOWN_bits(state, tracer, s_0_46);
        // S s_0_48: cast reint s_0_47 -> u64
        let s_0_48: u64 = (s_0_47.value() as u64);
        // S s_0_49: call Mk_BRBTGTINJ_EL1_Type(s_0_48)
        let s_0_49: ProductType5c790c8ef59cc8b2 = Mk_BRBTGTINJ_EL1_Type(
            state,
            tracer,
            s_0_48,
        );
        // C s_0_50: const #102304u : u32
        let s_0_50: u32 = 102304;
        // N s_0_51: write-reg s_0_50 <= s_0_49
        let s_0_51: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_50 as isize, s_0_49);
            tracer.write_register(s_0_50 as isize, s_0_49);
        };
        // C s_0_52: const #75u : u32
        let s_0_52: u32 = 75;
        // S s_0_53: call ConstrainUnpredictableBool(s_0_52)
        let s_0_53: bool = ConstrainUnpredictableBool(state, tracer, s_0_52);
        // N s_0_54: branch s_0_53 b2 b1
        if s_0_53 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #216u : u32
        let s_2_0: u32 = 216;
        // D s_2_1: read-reg s_2_0:u16
        let s_2_1: u16 = {
            let value = state.read_register::<u16>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call PMUEvent(s_2_1)
        let s_2_2: () = PMUEvent(state, tracer, s_2_1);
        // N s_2_3: return
        return;
    }
}
