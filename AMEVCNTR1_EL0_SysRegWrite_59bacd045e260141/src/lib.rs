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
use X_read::*;
use IsHighestEL::*;
use Mk_AMEVCNTR1_EL0_Type::*;
use IsG1ActivityMonitorImplemented::*;
use common::*;
pub fn AMEVCNTR1_EL0_SysRegWrite_59bacd045e260141<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #4s : i
        let s_0_0: i128 = 4;
        // S s_0_1: call IsG1ActivityMonitorImplemented(s_0_0)
        let s_0_1: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b4 b1
        if s_0_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call IsHighestEL(s_1_1)
        let s_1_2: bool = IsHighestEL(state, tracer, s_1_1);
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
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var t:i
        let s_3_1: i128 = fn_state.t;
        // D s_3_2: call X_read(s_3_1, s_3_0)
        let s_3_2: Bits = X_read(state, tracer, s_3_1, s_3_0);
        // D s_3_3: cast reint s_3_2 -> u64
        let s_3_3: u64 = (s_3_2.value() as u64);
        // D s_3_4: call Mk_AMEVCNTR1_EL0_Type(s_3_3)
        let s_3_4: ProductType5c790c8ef59cc8b2 = Mk_AMEVCNTR1_EL0_Type(
            state,
            tracer,
            s_3_3,
        );
        // C s_3_5: const #4s : i
        let s_3_5: i128 = 4;
        // C s_3_6: const #103240u : u32
        let s_3_6: u32 = 103240;
        // D s_3_7: read-reg s_3_6:[struct; 16]
        let s_3_7: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_3_6 as isize);
            tracer.read_register(s_3_6 as isize, value);
            value
        };
        // D s_3_8: mutate-element s_3_7[s_3_5] <= s_3_4
        let s_3_8: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut local = s_3_7.clone();
            local[(s_3_5) as usize] = s_3_4;
            local
        };
        // D s_3_9: cast cvt s_3_8 -> [struct; 0]
        let s_3_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_3_8,
        );
        // D s_3_10: cast cvt s_3_9 -> [struct; 16]
        let s_3_10: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_3_9);
            buf
        };
        // C s_3_11: const #103240u : u32
        let s_3_11: u32 = 103240;
        // N s_3_12: write-reg s_3_11 <= s_3_10
        let s_3_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_3_11 as isize, s_3_10);
            tracer.write_register(s_3_11 as isize, s_3_10);
        };
        // N s_3_13: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
