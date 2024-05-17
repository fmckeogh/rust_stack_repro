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
use u__id::*;
use GetBRBENumRecords::*;
use Mk_BRBSRCType::*;
use common::*;
pub fn UpdateBranchRecordBuffer<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ccu: bool,
    cc: u16,
    lastfailed: bool,
    transactional: bool,
    branch_type: u8,
    el: u8,
    mispredict: bool,
    valid_name: u8,
    source_address: u64,
    target_address: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        i: i64,
        ccu: bool,
        cc: u16,
        lastfailed: bool,
        transactional: bool,
        branch_type: u8,
        el: u8,
        mispredict: bool,
        valid_name: u8,
        source_address: u64,
        target_address: u64,
    }
    let fn_state = FunctionState {
        ccu,
        cc,
        lastfailed,
        transactional,
        branch_type,
        el,
        mispredict,
        valid_name,
        source_address,
        target_address,
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
        // S s_0_1: call GetBRBENumRecords(s_0_0)
        let s_0_1: i128 = GetBRBENumRecords(state, tracer, s_0_0);
        // C s_0_2: const #1s : i
        let s_0_2: i128 = 1;
        // S s_0_3: sub s_0_1 s_0_2
        let s_0_3: i128 = ((s_0_1) - (s_0_2));
        // S s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: write-var i <= s_0_4
        fn_state.i = s_0_4;
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #1s : i64
        let s_1_1: i64 = 1;
        // D s_1_2: cmp-lt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) < (s_1_1));
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
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
        // D s_2_7: read-var i:i64
        let s_2_7: i64 = fn_state.i;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: sub s_2_8 s_2_6
        let s_2_9: i128 = ((s_2_8) - (s_2_6));
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // C s_2_11: const #21240u : u32
        let s_2_11: u32 = 21240;
        // D s_2_12: read-reg s_2_11:[struct; 64]
        let s_2_12: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_11 as isize);
            tracer.read_register(s_2_11 as isize, value);
            value
        };
        // D s_2_13: cast zx s_2_10 -> i
        let s_2_13: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_14: read-element s_2_12[s_2_13]
        let s_2_14: ProductType5c790c8ef59cc8b2 = s_2_12[(s_2_13) as usize];
        // C s_2_15: const #21240u : u32
        let s_2_15: u32 = 21240;
        // D s_2_16: read-reg s_2_15:[struct; 64]
        let s_2_16: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_15 as isize);
            tracer.read_register(s_2_15 as isize, value);
            value
        };
        // D s_2_17: read-var i:i64
        let s_2_17: i64 = fn_state.i;
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_19: mutate-element s_2_16[s_2_18] <= s_2_14
        let s_2_19: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_16.clone();
            local[(s_2_18) as usize] = s_2_14;
            local
        };
        // D s_2_20: cast cvt s_2_19 -> [struct; 0]
        let s_2_20: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_19,
        );
        // D s_2_21: cast cvt s_2_20 -> [struct; 64]
        let s_2_21: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_20);
            buf
        };
        // C s_2_22: const #21240u : u32
        let s_2_22: u32 = 21240;
        // N s_2_23: write-reg s_2_22 <= s_2_21
        let s_2_23: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_22 as isize, s_2_21);
            tracer.write_register(s_2_22 as isize, s_2_21);
        };
        // C s_2_24: const #1s : i
        let s_2_24: i128 = 1;
        // D s_2_25: read-var i:i64
        let s_2_25: i64 = fn_state.i;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: sub s_2_26 s_2_24
        let s_2_27: i128 = ((s_2_26) - (s_2_24));
        // D s_2_28: cast reint s_2_27 -> i64
        let s_2_28: i64 = (s_2_27 as i64);
        // C s_2_29: const #101248u : u32
        let s_2_29: u32 = 101248;
        // D s_2_30: read-reg s_2_29:[struct; 64]
        let s_2_30: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_29 as isize);
            tracer.read_register(s_2_29 as isize, value);
            value
        };
        // D s_2_31: cast zx s_2_28 -> i
        let s_2_31: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_32: read-element s_2_30[s_2_31]
        let s_2_32: ProductType5c790c8ef59cc8b2 = s_2_30[(s_2_31) as usize];
        // C s_2_33: const #101248u : u32
        let s_2_33: u32 = 101248;
        // D s_2_34: read-reg s_2_33:[struct; 64]
        let s_2_34: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_33 as isize);
            tracer.read_register(s_2_33 as isize, value);
            value
        };
        // D s_2_35: read-var i:i64
        let s_2_35: i64 = fn_state.i;
        // D s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (i128::try_from(s_2_35).unwrap());
        // D s_2_37: mutate-element s_2_34[s_2_36] <= s_2_32
        let s_2_37: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_34.clone();
            local[(s_2_36) as usize] = s_2_32;
            local
        };
        // D s_2_38: cast cvt s_2_37 -> [struct; 0]
        let s_2_38: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_37,
        );
        // D s_2_39: cast cvt s_2_38 -> [struct; 64]
        let s_2_39: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_38);
            buf
        };
        // C s_2_40: const #101248u : u32
        let s_2_40: u32 = 101248;
        // N s_2_41: write-reg s_2_40 <= s_2_39
        let s_2_41: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_40 as isize, s_2_39);
            tracer.write_register(s_2_40 as isize, s_2_39);
        };
        // C s_2_42: const #1s : i
        let s_2_42: i128 = 1;
        // D s_2_43: read-var i:i64
        let s_2_43: i64 = fn_state.i;
        // D s_2_44: cast zx s_2_43 -> i
        let s_2_44: i128 = (i128::try_from(s_2_43).unwrap());
        // D s_2_45: sub s_2_44 s_2_42
        let s_2_45: i128 = ((s_2_44) - (s_2_42));
        // D s_2_46: cast reint s_2_45 -> i64
        let s_2_46: i64 = (s_2_45 as i64);
        // C s_2_47: const #100288u : u32
        let s_2_47: u32 = 100288;
        // D s_2_48: read-reg s_2_47:[struct; 64]
        let s_2_48: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_47 as isize);
            tracer.read_register(s_2_47 as isize, value);
            value
        };
        // D s_2_49: cast zx s_2_46 -> i
        let s_2_49: i128 = (i128::try_from(s_2_46).unwrap());
        // D s_2_50: read-element s_2_48[s_2_49]
        let s_2_50: ProductType5c790c8ef59cc8b2 = s_2_48[(s_2_49) as usize];
        // C s_2_51: const #100288u : u32
        let s_2_51: u32 = 100288;
        // D s_2_52: read-reg s_2_51:[struct; 64]
        let s_2_52: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_51 as isize);
            tracer.read_register(s_2_51 as isize, value);
            value
        };
        // D s_2_53: read-var i:i64
        let s_2_53: i64 = fn_state.i;
        // D s_2_54: cast zx s_2_53 -> i
        let s_2_54: i128 = (i128::try_from(s_2_53).unwrap());
        // D s_2_55: mutate-element s_2_52[s_2_54] <= s_2_50
        let s_2_55: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_2_52.clone();
            local[(s_2_54) as usize] = s_2_50;
            local
        };
        // D s_2_56: cast cvt s_2_55 -> [struct; 0]
        let s_2_56: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_2_55,
        );
        // D s_2_57: cast cvt s_2_56 -> [struct; 64]
        let s_2_57: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_56);
            buf
        };
        // C s_2_58: const #100288u : u32
        let s_2_58: u32 = 100288;
        // N s_2_59: write-reg s_2_58 <= s_2_57
        let s_2_59: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_58 as isize, s_2_57);
            tracer.write_register(s_2_58 as isize, s_2_57);
        };
        // D s_2_60: read-var i:i64
        let s_2_60: i64 = fn_state.i;
        // C s_2_61: const #1s : i64
        let s_2_61: i64 = 1;
        // D s_2_62: sub s_2_60 s_2_61
        let s_2_62: i64 = ((s_2_60) - (s_2_61));
        // D s_2_63: write-var i <= s_2_62
        fn_state.i = s_2_62;
        // N s_2_64: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // C s_3_1: const #100288u : u32
        let s_3_1: u32 = 100288;
        // D s_3_2: read-reg s_3_1:[struct; 64]
        let s_3_2: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_3_1 as isize);
            tracer.read_register(s_3_1 as isize, value);
            value
        };
        // D s_3_3: read-element s_3_2[s_3_0]
        let s_3_3: ProductType5c790c8ef59cc8b2 = s_3_2[(s_3_0) as usize];
        // C s_3_4: const #0s : i
        let s_3_4: i128 = 0;
        // C s_3_5: const #100288u : u32
        let s_3_5: u32 = 100288;
        // D s_3_6: read-reg s_3_5:[struct; 64]
        let s_3_6: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_3_5 as isize);
            tracer.read_register(s_3_5 as isize, value);
            value
        };
        // D s_3_7: mutate-element s_3_6[s_3_4] <= s_3_3
        let s_3_7: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_6.clone();
            local[(s_3_4) as usize] = s_3_3;
            local
        };
        // D s_3_8: cast cvt s_3_7 -> [struct; 0]
        let s_3_8: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_7,
        );
        // D s_3_9: cast cvt s_3_8 -> [struct; 64]
        let s_3_9: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_8);
            buf
        };
        // C s_3_10: const #100288u : u32
        let s_3_10: u32 = 100288;
        // N s_3_11: write-reg s_3_10 <= s_3_9
        let s_3_11: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_10 as isize, s_3_9);
            tracer.write_register(s_3_10 as isize, s_3_9);
        };
        // C s_3_12: const #0s : i
        let s_3_12: i128 = 0;
        // C s_3_13: const #100288u : u32
        let s_3_13: u32 = 100288;
        // D s_3_14: read-reg s_3_13:[struct; 64]
        let s_3_14: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_13 as isize);
            tracer.read_register(s_3_13 as isize, value);
            value
        };
        // D s_3_15: read-element s_3_14[s_3_12]
        let s_3_15: ProductType5c790c8ef59cc8b2 = s_3_14[(s_3_12) as usize];
        // C s_3_16: const #0s : i
        let s_3_16: i128 = 0;
        // C s_3_17: const #100288u : u32
        let s_3_17: u32 = 100288;
        // D s_3_18: read-reg s_3_17:[struct; 64]
        let s_3_18: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_17 as isize);
            tracer.read_register(s_3_17 as isize, value);
            value
        };
        // D s_3_19: mutate-element s_3_18[s_3_16] <= s_3_15
        let s_3_19: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_18.clone();
            local[(s_3_16) as usize] = s_3_15;
            local
        };
        // D s_3_20: cast cvt s_3_19 -> [struct; 0]
        let s_3_20: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_19,
        );
        // D s_3_21: cast cvt s_3_20 -> [struct; 64]
        let s_3_21: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_20);
            buf
        };
        // C s_3_22: const #100288u : u32
        let s_3_22: u32 = 100288;
        // N s_3_23: write-reg s_3_22 <= s_3_21
        let s_3_23: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_22 as isize, s_3_21);
            tracer.write_register(s_3_22 as isize, s_3_21);
        };
        // C s_3_24: const #0s : i
        let s_3_24: i128 = 0;
        // C s_3_25: const #100288u : u32
        let s_3_25: u32 = 100288;
        // D s_3_26: read-reg s_3_25:[struct; 64]
        let s_3_26: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_25 as isize);
            tracer.read_register(s_3_25 as isize, value);
            value
        };
        // D s_3_27: read-element s_3_26[s_3_24]
        let s_3_27: ProductType5c790c8ef59cc8b2 = s_3_26[(s_3_24) as usize];
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // C s_3_29: const #100288u : u32
        let s_3_29: u32 = 100288;
        // D s_3_30: read-reg s_3_29:[struct; 64]
        let s_3_30: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_29 as isize);
            tracer.read_register(s_3_29 as isize, value);
            value
        };
        // D s_3_31: mutate-element s_3_30[s_3_28] <= s_3_27
        let s_3_31: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_30.clone();
            local[(s_3_28) as usize] = s_3_27;
            local
        };
        // D s_3_32: cast cvt s_3_31 -> [struct; 0]
        let s_3_32: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_31,
        );
        // D s_3_33: cast cvt s_3_32 -> [struct; 64]
        let s_3_33: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_32);
            buf
        };
        // C s_3_34: const #100288u : u32
        let s_3_34: u32 = 100288;
        // N s_3_35: write-reg s_3_34 <= s_3_33
        let s_3_35: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_34 as isize, s_3_33);
            tracer.write_register(s_3_34 as isize, s_3_33);
        };
        // C s_3_36: const #0s : i
        let s_3_36: i128 = 0;
        // C s_3_37: const #100288u : u32
        let s_3_37: u32 = 100288;
        // D s_3_38: read-reg s_3_37:[struct; 64]
        let s_3_38: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_37 as isize);
            tracer.read_register(s_3_37 as isize, value);
            value
        };
        // D s_3_39: read-element s_3_38[s_3_36]
        let s_3_39: ProductType5c790c8ef59cc8b2 = s_3_38[(s_3_36) as usize];
        // C s_3_40: const #0s : i
        let s_3_40: i128 = 0;
        // C s_3_41: const #100288u : u32
        let s_3_41: u32 = 100288;
        // D s_3_42: read-reg s_3_41:[struct; 64]
        let s_3_42: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_41 as isize);
            tracer.read_register(s_3_41 as isize, value);
            value
        };
        // D s_3_43: mutate-element s_3_42[s_3_40] <= s_3_39
        let s_3_43: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_42.clone();
            local[(s_3_40) as usize] = s_3_39;
            local
        };
        // D s_3_44: cast cvt s_3_43 -> [struct; 0]
        let s_3_44: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_43,
        );
        // D s_3_45: cast cvt s_3_44 -> [struct; 64]
        let s_3_45: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_44);
            buf
        };
        // C s_3_46: const #100288u : u32
        let s_3_46: u32 = 100288;
        // N s_3_47: write-reg s_3_46 <= s_3_45
        let s_3_47: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_46 as isize, s_3_45);
            tracer.write_register(s_3_46 as isize, s_3_45);
        };
        // C s_3_48: const #0s : i
        let s_3_48: i128 = 0;
        // C s_3_49: const #100288u : u32
        let s_3_49: u32 = 100288;
        // D s_3_50: read-reg s_3_49:[struct; 64]
        let s_3_50: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_49 as isize);
            tracer.read_register(s_3_49 as isize, value);
            value
        };
        // D s_3_51: read-element s_3_50[s_3_48]
        let s_3_51: ProductType5c790c8ef59cc8b2 = s_3_50[(s_3_48) as usize];
        // C s_3_52: const #0s : i
        let s_3_52: i128 = 0;
        // C s_3_53: const #100288u : u32
        let s_3_53: u32 = 100288;
        // D s_3_54: read-reg s_3_53:[struct; 64]
        let s_3_54: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_53 as isize);
            tracer.read_register(s_3_53 as isize, value);
            value
        };
        // D s_3_55: mutate-element s_3_54[s_3_52] <= s_3_51
        let s_3_55: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_54.clone();
            local[(s_3_52) as usize] = s_3_51;
            local
        };
        // D s_3_56: cast cvt s_3_55 -> [struct; 0]
        let s_3_56: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_55,
        );
        // D s_3_57: cast cvt s_3_56 -> [struct; 64]
        let s_3_57: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_56);
            buf
        };
        // C s_3_58: const #100288u : u32
        let s_3_58: u32 = 100288;
        // N s_3_59: write-reg s_3_58 <= s_3_57
        let s_3_59: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_58 as isize, s_3_57);
            tracer.write_register(s_3_58 as isize, s_3_57);
        };
        // C s_3_60: const #0s : i
        let s_3_60: i128 = 0;
        // C s_3_61: const #100288u : u32
        let s_3_61: u32 = 100288;
        // D s_3_62: read-reg s_3_61:[struct; 64]
        let s_3_62: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_61 as isize);
            tracer.read_register(s_3_61 as isize, value);
            value
        };
        // D s_3_63: read-element s_3_62[s_3_60]
        let s_3_63: ProductType5c790c8ef59cc8b2 = s_3_62[(s_3_60) as usize];
        // C s_3_64: const #0s : i
        let s_3_64: i128 = 0;
        // C s_3_65: const #100288u : u32
        let s_3_65: u32 = 100288;
        // D s_3_66: read-reg s_3_65:[struct; 64]
        let s_3_66: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_65 as isize);
            tracer.read_register(s_3_65 as isize, value);
            value
        };
        // D s_3_67: mutate-element s_3_66[s_3_64] <= s_3_63
        let s_3_67: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_66.clone();
            local[(s_3_64) as usize] = s_3_63;
            local
        };
        // D s_3_68: cast cvt s_3_67 -> [struct; 0]
        let s_3_68: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_67,
        );
        // D s_3_69: cast cvt s_3_68 -> [struct; 64]
        let s_3_69: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_68);
            buf
        };
        // C s_3_70: const #100288u : u32
        let s_3_70: u32 = 100288;
        // N s_3_71: write-reg s_3_70 <= s_3_69
        let s_3_71: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_70 as isize, s_3_69);
            tracer.write_register(s_3_70 as isize, s_3_69);
        };
        // C s_3_72: const #0s : i
        let s_3_72: i128 = 0;
        // C s_3_73: const #100288u : u32
        let s_3_73: u32 = 100288;
        // D s_3_74: read-reg s_3_73:[struct; 64]
        let s_3_74: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_73 as isize);
            tracer.read_register(s_3_73 as isize, value);
            value
        };
        // D s_3_75: read-element s_3_74[s_3_72]
        let s_3_75: ProductType5c790c8ef59cc8b2 = s_3_74[(s_3_72) as usize];
        // C s_3_76: const #0s : i
        let s_3_76: i128 = 0;
        // C s_3_77: const #100288u : u32
        let s_3_77: u32 = 100288;
        // D s_3_78: read-reg s_3_77:[struct; 64]
        let s_3_78: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_77 as isize);
            tracer.read_register(s_3_77 as isize, value);
            value
        };
        // D s_3_79: mutate-element s_3_78[s_3_76] <= s_3_75
        let s_3_79: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_78.clone();
            local[(s_3_76) as usize] = s_3_75;
            local
        };
        // D s_3_80: cast cvt s_3_79 -> [struct; 0]
        let s_3_80: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_79,
        );
        // D s_3_81: cast cvt s_3_80 -> [struct; 64]
        let s_3_81: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_80);
            buf
        };
        // C s_3_82: const #100288u : u32
        let s_3_82: u32 = 100288;
        // N s_3_83: write-reg s_3_82 <= s_3_81
        let s_3_83: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_82 as isize, s_3_81);
            tracer.write_register(s_3_82 as isize, s_3_81);
        };
        // C s_3_84: const #0s : i
        let s_3_84: i128 = 0;
        // C s_3_85: const #100288u : u32
        let s_3_85: u32 = 100288;
        // D s_3_86: read-reg s_3_85:[struct; 64]
        let s_3_86: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_85 as isize);
            tracer.read_register(s_3_85 as isize, value);
            value
        };
        // D s_3_87: read-element s_3_86[s_3_84]
        let s_3_87: ProductType5c790c8ef59cc8b2 = s_3_86[(s_3_84) as usize];
        // C s_3_88: const #0s : i
        let s_3_88: i128 = 0;
        // C s_3_89: const #100288u : u32
        let s_3_89: u32 = 100288;
        // D s_3_90: read-reg s_3_89:[struct; 64]
        let s_3_90: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_89 as isize);
            tracer.read_register(s_3_89 as isize, value);
            value
        };
        // D s_3_91: mutate-element s_3_90[s_3_88] <= s_3_87
        let s_3_91: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_90.clone();
            local[(s_3_88) as usize] = s_3_87;
            local
        };
        // D s_3_92: cast cvt s_3_91 -> [struct; 0]
        let s_3_92: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_91,
        );
        // D s_3_93: cast cvt s_3_92 -> [struct; 64]
        let s_3_93: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_92);
            buf
        };
        // C s_3_94: const #100288u : u32
        let s_3_94: u32 = 100288;
        // N s_3_95: write-reg s_3_94 <= s_3_93
        let s_3_95: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_94 as isize, s_3_93);
            tracer.write_register(s_3_94 as isize, s_3_93);
        };
        // D s_3_96: read-var source_address:u64
        let s_3_96: u64 = fn_state.source_address;
        // D s_3_97: call Mk_BRBSRCType(s_3_96)
        let s_3_97: ProductType5c790c8ef59cc8b2 = Mk_BRBSRCType(state, tracer, s_3_96);
        // C s_3_98: const #0s : i
        let s_3_98: i128 = 0;
        // C s_3_99: const #21240u : u32
        let s_3_99: u32 = 21240;
        // D s_3_100: read-reg s_3_99:[struct; 64]
        let s_3_100: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_99 as isize);
            tracer.read_register(s_3_99 as isize, value);
            value
        };
        // D s_3_101: mutate-element s_3_100[s_3_98] <= s_3_97
        let s_3_101: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_100.clone();
            local[(s_3_98) as usize] = s_3_97;
            local
        };
        // D s_3_102: cast cvt s_3_101 -> [struct; 0]
        let s_3_102: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_101,
        );
        // D s_3_103: cast cvt s_3_102 -> [struct; 64]
        let s_3_103: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_102);
            buf
        };
        // C s_3_104: const #21240u : u32
        let s_3_104: u32 = 21240;
        // N s_3_105: write-reg s_3_104 <= s_3_103
        let s_3_105: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_104 as isize, s_3_103);
            tracer.write_register(s_3_104 as isize, s_3_103);
        };
        // D s_3_106: read-var target_address:u64
        let s_3_106: u64 = fn_state.target_address;
        // D s_3_107: call Mk_BRBTGTType(s_3_106)
        let s_3_107: ProductType5c790c8ef59cc8b2 = Mk_BRBTGTType(state, tracer, s_3_106);
        // C s_3_108: const #0s : i
        let s_3_108: i128 = 0;
        // C s_3_109: const #101248u : u32
        let s_3_109: u32 = 101248;
        // D s_3_110: read-reg s_3_109:[struct; 64]
        let s_3_110: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_109 as isize);
            tracer.read_register(s_3_109 as isize, value);
            value
        };
        // D s_3_111: mutate-element s_3_110[s_3_108] <= s_3_107
        let s_3_111: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut local = s_3_110.clone();
            local[(s_3_108) as usize] = s_3_107;
            local
        };
        // D s_3_112: cast cvt s_3_111 -> [struct; 0]
        let s_3_112: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_111,
        );
        // D s_3_113: cast cvt s_3_112 -> [struct; 64]
        let s_3_113: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_3_112);
            buf
        };
        // C s_3_114: const #101248u : u32
        let s_3_114: u32 = 101248;
        // N s_3_115: write-reg s_3_114 <= s_3_113
        let s_3_115: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_3_114 as isize, s_3_113);
            tracer.write_register(s_3_114 as isize, s_3_113);
        };
        // N s_3_116: return
        return;
    }
}
