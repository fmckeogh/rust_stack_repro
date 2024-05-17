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
use PhysMemWrite::*;
use IsFault::*;
use u__UNKNOWN_PhysMemRetStatus::*;
use AArch64_TranslateAddress::*;
use common::*;
pub fn DebugMemWrite<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    data: u8,
) -> ProductType7b38a52e3b2f4e94 {
    #[derive(Default)]
    struct FunctionState {
        memstatus: ProductTypef8c3639b88223255,
        addrdesc: ProductTypece7c66ccb2cab13e,
        return_value: ProductType7b38a52e3b2f4e94,
        vaddress: u64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        data: u8,
    }
    let fn_state = FunctionState {
        vaddress,
        accdesc,
        aligned,
        data,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType7b38a52e3b2f4e94 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call __UNKNOWN_PhysMemRetStatus(s_0_0)
        let s_0_1: ProductTypef8c3639b88223255 = u__UNKNOWN_PhysMemRetStatus(
            state,
            tracer,
            s_0_0,
        );
        // D s_0_2: write-var memstatus <= s_0_1
        fn_state.memstatus = s_0_1;
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-var vaddress:u64
        let s_0_5: u64 = fn_state.vaddress;
        // D s_0_6: read-var accdesc:struct
        let s_0_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_7: read-var aligned:u8
        let s_0_7: bool = fn_state.aligned;
        // D s_0_8: call AArch64_TranslateAddress(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_0_5,
            s_0_6,
            s_0_7,
            s_0_4,
        );
        // D s_0_9: write-var addrdesc <= s_0_8
        fn_state.addrdesc = s_0_8;
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType7b38a52e3b2f4e94 {
        // D s_1_0: read-var addrdesc:struct
        let s_1_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_1_1: call IsFault(s_1_0)
        let s_1_1: bool = IsFault(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b4 b2
        if s_1_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType7b38a52e3b2f4e94 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // D s_2_1: create-sum enum = 0:"s_2_0"
        let s_2_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_2_0);
        // C s_2_2: const #1s : i
        let s_2_2: i128 = 1;
        // D s_2_3: read-var data:u8
        let s_2_3: u8 = fn_state.data;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 8u16);
        // D s_2_5: read-var addrdesc:struct
        let s_2_5: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_2_6: read-var accdesc:struct
        let s_2_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_7: call PhysMemWrite(s_2_5, s_2_2, s_2_6, s_2_1, s_2_4)
        let s_2_7: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_2_5,
            s_2_2,
            s_2_6,
            s_2_1,
            s_2_4,
        );
        // D s_2_8: read-var addrdesc:struct
        let s_2_8: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_2_9: create-product struct = ["s_2_7", "s_2_8"]
        let s_2_9: ProductType7b38a52e3b2f4e94 = ProductType7b38a52e3b2f4e94 {
            _0: s_2_7,
            _1: s_2_8,
        };
        // D s_2_10: write-var return_value <= s_2_9
        fn_state.return_value = s_2_9;
        // N s_2_11: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType7b38a52e3b2f4e94 {
        // D s_3_0: read-var return_value:struct
        let s_3_0: ProductType7b38a52e3b2f4e94 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType7b38a52e3b2f4e94 {
        // D s_4_0: read-var memstatus:struct
        let s_4_0: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_4_1: read-var addrdesc:struct
        let s_4_1: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_4_2: create-product struct = ["s_4_0", "s_4_1"]
        let s_4_2: ProductType7b38a52e3b2f4e94 = ProductType7b38a52e3b2f4e94 {
            _0: s_4_0,
            _1: s_4_1,
        };
        // D s_4_3: write-var return_value <= s_4_2
        fn_state.return_value = s_4_2;
        // N s_4_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
