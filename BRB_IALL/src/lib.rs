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
use Mk_BRBTGTType::*;
use Mk_BRBINFType::*;
use u__id::*;
use Zeros::*;
use GetBRBENumRecords::*;
use Mk_BRBSRCType::*;
use common::*;
pub fn BRB_IALL<T: Tracer>(state: &mut State, tracer: &T, gs_26359: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        i: i64,
        gs_26363: i64,
        gs_26359: (),
    }
    let fn_state = FunctionState {
        gs_26359,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // C s_0_1: const #() : ()
        let s_0_1: () = ();
        // S s_0_2: call GetBRBENumRecords(s_0_1)
        let s_0_2: i128 = GetBRBENumRecords(state, tracer, s_0_1);
        // C s_0_3: const #1s : i
        let s_0_3: i128 = 1;
        // S s_0_4: sub s_0_2 s_0_3
        let s_0_4: i128 = ((s_0_2) - (s_0_3));
        // S s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: write-var gs#26363 <= s_0_5
        fn_state.gs_26363 = s_0_5;
        // D s_0_7: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_8: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#26363:i64
        let s_1_1: i64 = fn_state.gs_26363;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // C s_2_3: const #64s : i
        let s_2_3: i128 = 64;
        // D s_2_4: cmp-lt s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) < (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #64s : i
        let s_2_6: i128 = 64;
        // S s_2_7: call Zeros(s_2_6)
        let s_2_7: Bits = Zeros(state, tracer, s_2_6);
        // S s_2_8: cast reint s_2_7 -> u64
        let s_2_8: u64 = (s_2_7.value() as u64);
        // S s_2_9: call Mk_BRBSRCType(s_2_8)
        let s_2_9: ProductType5c790c8ef59cc8b2 = Mk_BRBSRCType(state, tracer, s_2_8);
        // C s_2_10: const #21240u : u32
        let s_2_10: u32 = 21240;
        // D s_2_11: read-reg s_2_10:[struct; 64]
        let s_2_11: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_10 as isize);
            tracer.read_register(s_2_10 as isize, value);
            value
        };
        // D s_2_12: read-var i:i64
        let s_2_12: i64 = fn_state.i;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: mutate-element s_2_11[s_2_13] <= s_2_9
        let s_2_14: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_11.clone();
            local[(s_2_13) as usize] = s_2_9;
            local
        };
        // D s_2_15: cast cvt s_2_14 -> [struct; 0]
        let s_2_15: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_14,
        );
        // D s_2_16: cast cvt s_2_15 -> [struct; 64]
        let s_2_16: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_15);
            buf
        };
        // C s_2_17: const #21240u : u32
        let s_2_17: u32 = 21240;
        // N s_2_18: write-reg s_2_17 <= s_2_16
        let s_2_18: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_17 as isize, s_2_16);
            tracer.write_register(s_2_17 as isize, s_2_16);
        };
        // C s_2_19: const #64s : i
        let s_2_19: i128 = 64;
        // S s_2_20: call Zeros(s_2_19)
        let s_2_20: Bits = Zeros(state, tracer, s_2_19);
        // S s_2_21: cast reint s_2_20 -> u64
        let s_2_21: u64 = (s_2_20.value() as u64);
        // S s_2_22: call Mk_BRBTGTType(s_2_21)
        let s_2_22: ProductType5c790c8ef59cc8b2 = Mk_BRBTGTType(state, tracer, s_2_21);
        // C s_2_23: const #101248u : u32
        let s_2_23: u32 = 101248;
        // D s_2_24: read-reg s_2_23:[struct; 64]
        let s_2_24: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_23 as isize);
            tracer.read_register(s_2_23 as isize, value);
            value
        };
        // D s_2_25: read-var i:i64
        let s_2_25: i64 = fn_state.i;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: mutate-element s_2_24[s_2_26] <= s_2_22
        let s_2_27: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_24.clone();
            local[(s_2_26) as usize] = s_2_22;
            local
        };
        // D s_2_28: cast cvt s_2_27 -> [struct; 0]
        let s_2_28: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_27,
        );
        // D s_2_29: cast cvt s_2_28 -> [struct; 64]
        let s_2_29: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_28);
            buf
        };
        // C s_2_30: const #101248u : u32
        let s_2_30: u32 = 101248;
        // N s_2_31: write-reg s_2_30 <= s_2_29
        let s_2_31: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_30 as isize, s_2_29);
            tracer.write_register(s_2_30 as isize, s_2_29);
        };
        // C s_2_32: const #64s : i
        let s_2_32: i128 = 64;
        // S s_2_33: call Zeros(s_2_32)
        let s_2_33: Bits = Zeros(state, tracer, s_2_32);
        // S s_2_34: cast reint s_2_33 -> u64
        let s_2_34: u64 = (s_2_33.value() as u64);
        // S s_2_35: call Mk_BRBINFType(s_2_34)
        let s_2_35: ProductType5c790c8ef59cc8b2 = Mk_BRBINFType(state, tracer, s_2_34);
        // C s_2_36: const #100288u : u32
        let s_2_36: u32 = 100288;
        // D s_2_37: read-reg s_2_36:[struct; 64]
        let s_2_37: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_36 as isize);
            tracer.read_register(s_2_36 as isize, value);
            value
        };
        // D s_2_38: read-var i:i64
        let s_2_38: i64 = fn_state.i;
        // D s_2_39: cast zx s_2_38 -> i
        let s_2_39: i128 = (i128::try_from(s_2_38).unwrap());
        // D s_2_40: mutate-element s_2_37[s_2_39] <= s_2_35
        let s_2_40: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_37.clone();
            local[(s_2_39) as usize] = s_2_35;
            local
        };
        // D s_2_41: cast cvt s_2_40 -> [struct; 0]
        let s_2_41: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_40,
        );
        // D s_2_42: cast cvt s_2_41 -> [struct; 64]
        let s_2_42: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_41);
            buf
        };
        // C s_2_43: const #100288u : u32
        let s_2_43: u32 = 100288;
        // N s_2_44: write-reg s_2_43 <= s_2_42
        let s_2_44: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_43 as isize, s_2_42);
            tracer.write_register(s_2_43 as isize, s_2_42);
        };
        // D s_2_45: read-var i:i64
        let s_2_45: i64 = fn_state.i;
        // C s_2_46: const #1s : i64
        let s_2_46: i64 = 1;
        // D s_2_47: add s_2_45 s_2_46
        let s_2_47: i64 = (s_2_45 + s_2_46);
        // D s_2_48: write-var i <= s_2_47
        fn_state.i = s_2_47;
        // N s_2_49: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
}
